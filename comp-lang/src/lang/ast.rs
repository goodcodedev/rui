#[derive(Debug)]
pub struct ActionItem<'a> {
    pub ident: &'a str,
}

#[derive(Debug)]
pub struct Actions<'a> {
    pub action_items: Vec<ActionItem<'a>>,
}

#[derive(Debug)]
pub struct Comp<'a> {
    pub items: Vec<SourceItem<'a>>,
}

#[derive(Debug)]
pub struct Content<'a> {
    pub content: &'a str,
}

#[derive(Debug)]
pub struct IdentAttrib<'a> {
    pub ident: &'a str,
    pub name: &'a str,
}

#[derive(Debug)]
pub struct PropDecl<'a> {
    pub typ: Typ,
    pub ident: &'a str,
}

#[derive(Debug)]
pub struct StringAttrib<'a> {
    pub name: &'a str,
    pub string: &'a str,
}

#[derive(Debug)]
pub struct TagElement<'a> {
    pub end_tag: &'a str,
    pub ident: &'a str,
    pub tag_attributes: Vec<TagAttribute<'a>>,
    pub tag_items: Vec<TagItem<'a>>,
}

#[derive(Debug)]
pub enum SourceItem<'a> {
    PropDeclItem(PropDecl<'a>),
    ActionsItem(Actions<'a>),
    TagElementItem(TagElement<'a>),
}

#[derive(Debug)]
pub enum TagAttribute<'a> {
    StringAttribItem(StringAttrib<'a>),
    IdentAttribItem(IdentAttrib<'a>),
}

#[derive(Debug)]
pub enum TagItem<'a> {
    TagElementItem(TagElement<'a>),
    ContentItem(Content<'a>),
}

#[derive(Debug)]
pub enum Typ {
    U32,
    StringType,
    BoolType,
}

