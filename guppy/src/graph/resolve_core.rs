// Copyright (c) The cargo-guppy Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::graph::select_core::{all_visit_map, reachable_map, SelectParams};
use crate::graph::{DependencyDirection, GraphSpec};
use crate::petgraph_support::scc::{NodeIter, Sccs};
use fixedbitset::FixedBitSet;
use petgraph::prelude::*;
use petgraph::visit::{NodeFiltered, Reversed, VisitMap};
use serde::export::PhantomData;

/// Core logic for select queries that have been resolved into a known set of packages.
///
/// The `G` param ensures that package and feature resolutions aren't mixed up accidentally.
#[derive(Clone, Debug)]
pub(super) struct ResolveCore<G> {
    pub(super) included: FixedBitSet,
    pub(super) count: usize,
    _phantom: PhantomData<G>,
}

impl<G: GraphSpec> ResolveCore<G> {
    pub(super) fn new(
        graph: &Graph<G::Node, G::Edge, Directed, G::Ix>,
        params: SelectParams<G>,
    ) -> Self {
        let (included, count) = match params {
            SelectParams::All => all_visit_map(graph),
            SelectParams::SelectForward(initials) => reachable_map(graph, initials),
            SelectParams::SelectReverse(initials) => reachable_map(Reversed(graph), initials),
        };
        Self {
            included,
            count,
            _phantom: PhantomData,
        }
    }

    /// Returns the root metadatas in the specified direction.
    pub(super) fn roots(
        &self,
        graph: &Graph<G::Node, G::Edge, Directed, G::Ix>,
        sccs: &Sccs<G::Ix>,
        direction: DependencyDirection,
    ) -> Vec<NodeIndex<G::Ix>> {
        // If any element of an SCC is in the reachable map, so would every other element. This
        // means that any SCC map computed on the full graph will work on a prefiltered graph. (This
        // will change if we decide to implement edge visiting/filtering.)
        //
        // TODO: petgraph 0.5.1 will allow the closure to be replaced with &self.reachable. Switch
        // to it when it's out.
        match direction {
            DependencyDirection::Forward => sccs
                .externals(&NodeFiltered::from_fn(graph, |x| {
                    self.included.is_visited(&x)
                }))
                .collect(),
            DependencyDirection::Reverse => sccs
                .externals(&NodeFiltered::from_fn(Reversed(graph), |x| {
                    self.included.is_visited(&x)
                }))
                .collect(),
        }
    }

    pub(super) fn topo(self, sccs: &Sccs<G::Ix>, direction: DependencyDirection) -> Topo<G> {
        // ---
        // IMPORTANT
        // ---
        //
        // This uses the same list of sccs that's computed for the entire graph. This is *currently*
        // fine because with our current filters, if one element of an SCC is present all others
        // will be present as well.
        //
        // However:
        // * If we allow for custom visitors that can control the reachable map, it is possible that
        //   SCCs in the main graph aren't in the subgraph. That might make the returned order
        //   incorrect.
        // * This requires iterating over every node in the graph even if the set of returned nodes
        //   is very small. There's a tradeoff here between allocating memory to store a custom list
        //   of SCCs and just using the one available. More benchmarking is required to figure out
        //   the best approach.
        //
        // Note that the SCCs can be computed in reachable_map by adapting parts of kosaraju_scc.
        let node_iter = sccs.node_iter(direction.into());

        Topo {
            node_iter,
            included: self.included,
            remaining: self.count,
        }
    }
}

/// An iterator over package nodes in topological order.
#[derive(Clone, Debug)]
pub(super) struct Topo<'g, G: GraphSpec> {
    node_iter: NodeIter<'g, G::Ix>,
    included: FixedBitSet,
    remaining: usize,
}

impl<'g, G: GraphSpec> Topo<'g, G> {
    /// Returns the direction the iteration is happening in.
    pub fn direction(&self) -> DependencyDirection {
        self.node_iter.direction().into()
    }
}

impl<'g, G: GraphSpec> Iterator for Topo<'g, G> {
    type Item = NodeIndex<G::Ix>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(ix) = self.node_iter.next() {
            if !self.included.is_visited(&ix) {
                continue;
            }
            self.remaining -= 1;
            return Some(ix);
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<'g, G: GraphSpec> ExactSizeIterator for Topo<'g, G> {
    fn len(&self) -> usize {
        self.remaining
    }
}