pub trait Graph<'a, N: 'a> {

    type NeighbourIterator: IntoIterator<Item = &'a N>;

    fn adjacent(node: &N, node: &N) -> bool;

    fn neighbours(&self, node: &N) -> Self::NeighbourIterator;

}