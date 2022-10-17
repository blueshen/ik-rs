use crate::core::lexeme::Lexeme;
use crate::core::lexeme_path::LexemePath;
use crate::core::ordered_linked_list::{Node, OrderedLinkedList};
use std::collections::{BTreeSet, HashMap};
use std::ptr::NonNull;
use crate::core::ik_segmenter::TokenMode;

// IK分词歧义裁决器
pub struct IKArbitrator {}

impl IKArbitrator {
    pub fn new() -> Self {
        IKArbitrator {}
    }

    // 分词歧义处理
    pub unsafe fn process(
        &mut self,
        org_lexemes: &mut OrderedLinkedList<Lexeme>,
        mode: TokenMode,
    ) -> HashMap<usize, LexemePath> {
        // org_lexemes.traverse();
        let mut path_map = HashMap::<usize, LexemePath>::new();
        let mut cross_path = LexemePath::new();
        let mut cur_node = org_lexemes.head_node();
        while cur_node.is_some() {
            let org_lexeme = &(cur_node.as_ref().unwrap().as_ref().val);
            if !cross_path.add_cross_lexeme(org_lexeme) {
                //找到与crossPath不相交的下一个crossPath
                if cross_path.size() == 1 || !(mode == TokenMode::SEARCH) {
                    //crossPath没有歧义 或者 不做歧义处理
                    //直接输出当前crossPath
                    path_map.insert(cross_path.get_path_begin() as usize, cross_path);
                } else {
                    //对当前的crossPath进行歧义处理
                    let judge_result = self.judge(cur_node);
                    //输出歧义处理结果judgeResult
                    path_map.insert(
                        judge_result.as_ref().unwrap().get_path_begin() as usize,
                        judge_result.unwrap(),
                    );
                }
                //把orgLexeme加入新的crossPath中
                cross_path = LexemePath::new();
                cross_path.add_cross_lexeme(org_lexeme);
            }
            cur_node = cur_node.as_ref().unwrap().as_ref().next.as_ref();
        }

        //处理最后的path
        if cross_path.size() == 1 || !(mode == TokenMode::SEARCH) {
            // crossPath没有歧义 或者 不做歧义处理
            // 直接输出当前crossPath
            path_map.insert(cross_path.get_path_begin() as usize, cross_path);
        } else {
            // 对当前的crossPath进行歧义处理
            let judge_result = self.judge(cross_path.get_head());
            // 输出歧义处理结果judgeResult
            path_map.insert(
                judge_result.as_ref().unwrap().get_path_begin() as usize,
                judge_result.unwrap(),
            );
        }
        path_map
    }

    /**
     * 歧义识别
     *
     * @param lexeme_cell     歧义路径链表头
     * @param fullTextLength 歧义路径文本长度
     */
    pub unsafe fn judge(&mut self, cur_node: Option<&NonNull<Node<Lexeme>>>) -> Option<LexemePath> {
        //候选路径集合
        let mut path_options = BTreeSet::new();
        //候选结果路径
        let mut option = LexemePath::new();
        //对crossPath进行一次遍历,同时返回本次遍历中有冲突的Lexeme栈
        let mut lexeme_stack = self.forward_path(cur_node, &mut option);

        //当前词元链并非最理想的，加入候选路径集合
        path_options.insert(option.clone());

        //存在歧义词，处理
        let mut c;
        while !lexeme_stack.is_empty() {
            c = lexeme_stack.pop();
            //回滚词元链
            self.back_path(c.unwrap(), &mut option);
            //从歧义词位置开始，递归，生成可选方案
            self.forward_path(c.unwrap(), &mut option);
            path_options.insert(option.clone());
        }
        //返回集合中的最优方案
        let mut a = None;
        if !path_options.is_empty() {
            for o in path_options.iter() {
                a = Some(o.clone());
                break;
            }
        }
        return a;
    }

    // 向前遍历，添加词元，构造一个无歧义词元组合
    pub unsafe fn forward_path<'a>(
        &'a self,
        cur_node: Option<&'a NonNull<Node<Lexeme>>>,
        option: &mut LexemePath,
    ) -> Vec<Option<&NonNull<Node<Lexeme>>>> {
        //发生冲突的Lexeme栈
        let mut conflict_stack: Vec<Option<&NonNull<Node<Lexeme>>>> = Vec::new();
        //迭代遍历Lexeme链表
        let mut cur = cur_node;
        while cur.is_some() {
            let c = &(cur.as_ref().unwrap().as_ref().val);
            if !option.add_not_cross_lexeme(c) {
                //词元交叉，添加失败则加入lexemeStack栈
                conflict_stack.push(cur);
            }
            cur = cur.as_ref().unwrap().as_ref().next.as_ref();
        }
        return conflict_stack;
    }

    // 回滚词元链，直到它能够接受指定的词元
    pub unsafe fn back_path(&self, l: Option<&NonNull<Node<Lexeme>>>, option: &mut LexemePath) {
        let lexeme = &(l.as_ref().unwrap().as_ref().val);
        while option.check_cross(lexeme) {
            option.remove_tail();
        }
    }
}
