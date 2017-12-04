use super::ast::*;

pub trait Visitor<'a> {
    fn visit_action_item(&mut self, node: &'a ActionItem) {
    }

    fn visit_actions(&mut self, node: &'a Actions) {
        for item in &node.action_items {
            self.visit_action_item(item);
        }
    }

    fn visit_comp(&mut self, node: &'a Comp) {
        for item in &node.items {
            self.visit_source_item(item);
        }
    }

    fn visit_content(&mut self, node: &'a Content) {
    }

    fn visit_ident_attrib(&mut self, node: &'a IdentAttrib) {
    }

    fn visit_prop_decl(&mut self, node: &'a PropDecl) {
        self.visit_typ(&node.typ);
    }

    fn visit_string_attrib(&mut self, node: &'a StringAttrib) {
    }

    fn visit_tag_element(&mut self, node: &'a TagElement) {
        for item in &node.tag_attributes {
            self.visit_tag_attribute(item);
        }
        for item in &node.tag_items {
            self.visit_tag_item(item);
        }
    }

    fn visit_source_item(&mut self, node: &'a SourceItem) {
        match node {
            &SourceItem::PropDeclItem(ref inner) => self.visit_prop_decl(inner),
            &SourceItem::ActionsItem(ref inner) => self.visit_actions(inner),
            &SourceItem::TagElementItem(ref inner) => self.visit_tag_element(inner),
        }
    }

    fn visit_tag_attribute(&mut self, node: &'a TagAttribute) {
        match node {
            &TagAttribute::StringAttribItem(ref inner) => self.visit_string_attrib(inner),
            &TagAttribute::IdentAttribItem(ref inner) => self.visit_ident_attrib(inner),
        }
    }

    fn visit_tag_item(&mut self, node: &'a TagItem) {
        match node {
            &TagItem::TagElementItem(ref inner) => self.visit_tag_element(inner),
            &TagItem::ContentItem(ref inner) => self.visit_content(inner),
        }
    }

    fn visit_typ(&mut self, node: &'a Typ) {
    }

}