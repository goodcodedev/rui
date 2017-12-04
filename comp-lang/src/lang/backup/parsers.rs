use descr_common::parsers::*;
extern crate nom;
use self::nom::*;
use std;
use super::ast::*;

named!(pub start<Comp>, do_parse!(res: comp >> (res)));

named!(pub actions<Actions>,
    do_parse!(
        sp >> tag!("actions") >>
        sp >> char!('{') >>
        sp >> action_items_k: action_items >>
        sp >> char!('}') >>
        (Actions {
            action_items: action_items_k,
        }))
);

named!(pub comp<Comp>,
    do_parse!(
        sp >> items_k: source_items >>
        (Comp {
            items: items_k,
        }))
);

named!(pub prop_decl<PropDecl>,
    do_parse!(
        sp >> tag!("@") >>
        sp >> ident_k: ident >>
        sp >> char!(':') >>
        sp >> typ_k: typ >>
        (PropDecl {
            ident: ident_k,
            typ: typ_k,
        }))
);

named!(pub tag_element<TagElement>,
    do_parse!(
        sp >> tag!("<") >>
        sp >> ident_k: ident >>
        sp >> tag_attributes_k: tag_attributes >>
        sp >> tag!(">") >>
        sp >> tag_items_k: tag_items >>
        sp >> tag!("</") >>
        sp >> end_tag_k: ident >>
        sp >> tag!(">") >>
        (TagElement {
            ident: ident_k,
            tag_attributes: tag_attributes_k,
            tag_items: tag_items_k,
            end_tag: end_tag_k,
        }))
);

named!(pub typ<Typ>, alt_complete!(
    do_parse!(
        sp >> tag!("u32") >>
        (Typ::U32        ))
    | do_parse!(
        sp >> tag!("string") >>
        (Typ::StringType        ))
    | do_parse!(
        sp >> tag!("bool") >>
        (Typ::BoolType        ))
));

named!(pub action_items<Vec<ActionItem>>, many0!(
    do_parse!(
        sp >> ident_k: ident >>
        (ActionItem {
            ident: ident_k,
        }))
));

named!(pub source_items<Vec<SourceItem>>, many0!(alt_complete!(
    map!(prop_decl, |node| { SourceItem::PropDeclItem(node) })
    | map!(actions, |node| { SourceItem::ActionsItem(node) })
    | map!(tag_element, |node| { SourceItem::TagElementItem(node) })
)));

named!(pub tag_attributes<Vec<TagAttribute>>, separated_list!(sp, alt_complete!(
    do_parse!(
        sp >> name_k: ident >>
        sp >> char!('=') >>
        sp >> string_k: quoted_str >>
        (TagAttribute::StringAttribItem(StringAttrib {
            name: name_k,
            string: string_k,
        })))
    | do_parse!(
        sp >> name_k: ident >>
        sp >> char!('=') >>
        sp >> ident_k: ident >>
        (TagAttribute::IdentAttribItem(IdentAttrib {
            name: name_k,
            ident: ident_k,
        })))
)));

named!(pub tag_items<Vec<TagItem>>, separated_list!(sp, alt_complete!(
    map!(tag_element, |node| { TagItem::TagElementItem(node) })
    | do_parse!(
        content_k: until_done_result!(tag!("<")) >>
        (TagItem::ContentItem(Content {
            content: std::str::from_utf8(content_k).unwrap(),
        })))
)));

