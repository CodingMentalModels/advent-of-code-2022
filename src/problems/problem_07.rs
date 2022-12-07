use crate::input::input::InputParser;

pub fn solve_problem_07a(input: Vec<String>) -> usize {
    unimplemented!();
}

fn solve_problem_07b(input: Vec<String>) -> usize {
    unimplemented!();
}

enum FileTreeNode {
    File(String, usize),
    Directory(String, Vec<FileTreeNode>),
}

impl FileTreeNode {

    pub fn file(name: String, size: usize) -> Self {
        Self::File(name, size)
    }

    pub fn directory(name: String, children: Vec<Self>) -> Self {
        assert!(children.len() > 0);
        Self::Directory(name, children)
    }

    pub fn len(&self) -> usize {
        match self {
            Self::File(_, _) => { 1 },
            Self::Directory(_, children) => { 1 + children.iter().map(|c| c.len()).sum::<usize>() }
        }
    }

    pub fn depth(&self) -> usize {
        match self {
            Self::File(_, _) => { 1 },
            Self::Directory(_, children) => { 1 + children.iter().map(|c| c.depth()).max().expect("All branches should have children") }
        }
    }

    pub fn sum_sizes_if(&self, f: fn(usize) -> bool) -> usize {
        match self {
            Self::File(_, size) => { if f(*size) {*size} else {0} },
            Self::Directory(_, children) => { children.iter().map(|c| c.sum_sizes_if(f)).sum() }
        }
    }

    pub fn sum_sizes(&self) -> usize {
        self.sum_sizes_if(|n| true)
    }
    
}

#[cfg(test)]
mod test_problem_07 {

    use super::*;

    #[test]
    fn test_problem_07a_passes() {
        
        let example = InputParser::new().parse_as_string("example_input_07.txt").unwrap();

        let answer = solve_problem_07a(example);
        assert_eq!(answer, 95437);

        let input = InputParser::new().parse_as_string("input_07.txt").unwrap();

        let answer = solve_problem_07a(input);
        assert_eq!(answer, 0);
    }
    
    #[test]
    fn test_problem_07b_passes() {
        let input = InputParser::new().parse_as_string("input_07.txt").unwrap();
        let shorted_input = input.iter().take(10).map(|i| i.clone()).collect();

        assert_eq!(solve_problem_07b(shorted_input), 0);

        let answer = solve_problem_07b(input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_filetree_constructs() {
        let children = (1..4).into_iter().map(|i| FileTreeNode::file(format!("Child_{}", i), i)).collect();
        let tree = FileTreeNode::directory("tree".to_string(), children);
        assert_eq!(tree.len(), 4);
        assert_eq!(tree.depth(), 2);
        assert_eq!(tree.sum_sizes(), 6);
        assert_eq!(tree.sum_sizes_if(|size| size <= 2), 3);
    }
}