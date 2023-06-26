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
        org_lexemes: &mut OrderedLinkedList<Lexeme>,
        mode: TokenMode,
    ) -> HashMap<usize, LexemePath> {
        let mut path_map = HashMap::<usize, LexemePath>::new();
        let mut cross_path = LexemePath::new();
        for org_lexeme in org_lexemes.iter() {
            if !cross_path.add_cross_lexeme(org_lexeme) {
                if self.need_add_path(&cross_path, mode) {
                    path_map.insert(cross_path.path_begin() as usize, cross_path);
                } else {
                    let judge_result = self.judge(cross_path.head_node());
                    if let Some(path) = judge_result {
                        path_map.insert(path.path_begin() as usize, path);
                    }
                }
                cross_path = LexemePath::new();
                cross_path.add_cross_lexeme(org_lexeme);
            }
        }
        if self.need_add_path(&cross_path, mode) {
            path_map.insert(cross_path.path_begin() as usize, cross_path);
        } else {
            let judge_result = self.judge(cross_path.head_node());
            if let Some(path) = judge_result {
                path_map.insert(path.path_begin() as usize, path);
            }
        }
        path_map
    }

    fn judge(&mut self, cur_node: Option<&Link<Lexeme>>) -> Option<LexemePath> {
        let mut path_options = BTreeSet::new();
        let mut option = LexemePath::new();
        let mut lexeme_stack = self.forward_path(cur_node, &mut option);
        path_options.insert(option.clone());
        while let Some(node) = lexeme_stack.pop() {
            self.back_path(node, &mut option);
            self.forward_path(node, &mut option);
            path_options.insert(option.clone());
        }
        // pick first one
        let mut best = None;
        if !path_options.is_empty() {
            for o in path_options.into_iter() {
                best = Some(o);
                break;
            }
        }
        best
    }

    fn need_add_path(&self, cross_path: &LexemePath, mode: TokenMode) -> bool {
        cross_path.size() == 1 || !(mode == TokenMode::SEARCH)
    }

    fn forward_path<'a>(
        &'a self,
        cur_node: Option<&'a Link<Lexeme>>,
        option: &mut LexemePath,
    ) -> Vec<Option<&Link<Lexeme>>> {
        let mut conflict_stack: Vec<Option<&Link<Lexeme>>> = Vec::new();
        let mut cur = cur_node;
        while let Some(node) = cur {
            let ref_node = unsafe { node.as_ref() }; // safety
            let c = ref_node.ref_val();
            if !option.add_not_cross_lexeme(c) {
                conflict_stack.push(cur);
            }
            cur = ref_node.next.as_ref();
        }
        conflict_stack
    }

    fn back_path(&self, l: Option<&Link<Lexeme>>, option: &mut LexemePath) {
        if let Some(node) = l {
            let ref_node = unsafe { node.as_ref() }; // safety
            let lexeme = ref_node.ref_val();
            while option.check_cross(lexeme) {
                option.remove_tail();
            }
        }
    }
}
