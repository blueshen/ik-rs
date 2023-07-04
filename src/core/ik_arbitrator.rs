use crate::core::ik_segmenter::TokenMode;
use crate::core::lexeme::Lexeme;
use crate::core::lexeme_path::LexemePath;
use crate::core::ordered_linked_list::{Link, OrderedLinkedList};
use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
pub struct IKArbitrator {}
impl IKArbitrator {
    pub fn process(
        &mut self,
        orgin_lexemes: &OrderedLinkedList<Lexeme>,
        mode: &TokenMode,
    ) -> HashMap<usize, LexemePath> {
        let mut path_map = HashMap::<usize, LexemePath>::new();
        let mut cross_path = LexemePath::new();
        for org_lexeme in orgin_lexemes.iter() {
            if !cross_path.add_cross_lexeme(org_lexeme) {
                if self.need_add_path(&cross_path, mode) {
                    path_map.insert(cross_path.begin() as usize, cross_path);
                } else {
                    let judge_result = self.judge(cross_path.head_node());
                    if let Some(path) = judge_result {
                        path_map.insert(path.begin() as usize, path);
                    }
                }
                cross_path = LexemePath::new();
                cross_path.add_cross_lexeme(org_lexeme);
            }
        }
        if self.need_add_path(&cross_path, mode) {
            path_map.insert(cross_path.begin() as usize, cross_path);
        } else {
            let judge_result = self.judge(cross_path.head_node());
            if let Some(path) = judge_result {
                path_map.insert(path.begin() as usize, path);
            }
        }
        path_map
    }

    fn judge(&self, cur_node: Option<&Link<Lexeme>>) -> Option<LexemePath> {
        let mut path_options = BTreeSet::new();
        let mut option_path = LexemePath::new();
        let mut lexeme_stack = self.forward_path(cur_node, &mut option_path);
        path_options.insert(option_path.clone());
        while let Some(node) = lexeme_stack.pop() {
            self.back_path(node, &mut option_path);
            self.forward_path(node, &mut option_path);
            path_options.insert(option_path.clone());
        }
        // pick first one as best
        let mut best_path = None;
        if !path_options.is_empty() {
            for o in path_options.into_iter() {
                best_path = Some(o);
                break;
            }
        }
        best_path
        // after rust 1.66.0
        // path_options.pop_first()
    }

    fn need_add_path(&self, cross_path: &LexemePath, mode: &TokenMode) -> bool {
        match mode {
            TokenMode::INDEX => return true,
            _ => {}
        }
        cross_path.len() == 1
    }

    fn forward_path<'a>(
        &'a self,
        cur_node: Option<&'a Link<Lexeme>>,
        option_path: &mut LexemePath,
    ) -> Vec<Option<&Link<Lexeme>>> {
        let mut conflict_stack: Vec<Option<&Link<Lexeme>>> = Vec::new();
        let mut cur = cur_node;
        while let Some(node) = cur {
            let ref_node = unsafe { node.as_ref() }; // safety
            let c = ref_node.ref_val();
            if !option_path.add_not_cross_lexeme(c) {
                conflict_stack.push(cur);
            }
            cur = ref_node.next.as_ref();
        }
        conflict_stack
    }

    fn back_path(&self, l: Option<&Link<Lexeme>>, option_path: &mut LexemePath) {
        if let Some(node) = l {
            let ref_node = unsafe { node.as_ref() }; // safety
            let lexeme = ref_node.ref_val();
            while option_path.check_cross(lexeme) {
                option_path.remove_tail();
            }
        }
    }
}
