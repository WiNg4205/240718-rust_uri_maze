pub struct BST {
    root: TreeNode
}

pub struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
    value: char
}

impl TreeNode {
    pub fn new(value: char) -> Self {
        Self {
            left: None,
            right: None,
            value: value,
        }
    }

    pub fn add_left(&mut self, value: char) {
        self.left = Some(Box::new(TreeNode::new(value)));
    }

    pub fn add_right(&mut self, value: char) {
        self.right = Some(Box::new(TreeNode::new(value)));
    }

    pub fn get_left_mut(&mut self) -> &mut TreeNode {
        match self.left {
            Some(ref mut btn) => {
                btn.as_mut()
            },
            None => self,
        }
    }

    pub fn get_right_mut(&mut self) -> &mut TreeNode {
        match self.right {
            Some(ref mut btn) => {
                btn.as_mut()
            },
            None => self,
        }
    }

    pub fn get_left(&self) -> Option<&TreeNode> {
        match self.left {
            Some(ref btn) => {
                Some(btn.as_ref())
            },
            None => None
        }
    }

    pub fn get_right(&self) -> Option<&TreeNode> {
        match self.right {
            Some(ref btn) => {
                Some(btn.as_ref())
            },
            None => None
        }
    }

    pub fn get_value(&self) -> char {
        self.value
    }
}

impl BST {
    pub fn new(value: char) -> Self {
        Self {
            root: TreeNode::new(value)
        }
    }

    pub fn get_root_mut(&mut self) -> &mut TreeNode {
        &mut self.root
    }

    pub fn get_root(&self) -> &TreeNode {
        &self.root
    }
}
