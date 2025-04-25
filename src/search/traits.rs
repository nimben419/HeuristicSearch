pub trait StateSpaceProblem {
    type State: Eq + std::hash::Hash + Clone + std::fmt::Debug;
    type Action: Clone + std::fmt::Debug;
    type Cost;

    // Rust では get prefixは推奨されないため、教科書とは異なる命名規則を使用
    fn initial_state(&self) -> Self::State;
    fn is_goal(&self, state: &Self::State) -> bool;
    fn actions(&self, state: &Self::State) -> Vec<Self::Action>; // 可能な行動を取得
    fn transition(&self, state: &Self::State, action: &Self::Action) -> Self::State; // 遷移先を取得
    fn cost(&self, state: &Self::State, action: &Self::Action) -> Self::Cost; // 行動のコストを取得
    fn heuristic(&self, state: &Self::State) -> Self::Cost;
}
