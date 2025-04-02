use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;
use std::ops::*;
use std::fs;
use std::path::{Path, PathBuf};


impl AssetStructGenerator
{
    pub fn add_default_extensions_and_tag(&mut self) -> &mut Self
    {
        self.add_extension(AssetExtension::new("png", "Texture2D", AssetsKind::Texture2D));
        self.add_extension(AssetExtension::new("ttf", "Font", AssetsKind::Font));
        // mp3 is not supported on windows...
        //self.add_extension(AssetExtension::new("mp3", "Sound", AssetsKind::Audio));
        self.add_extension(AssetExtension::new("wav", "Sound", AssetsKind::Audio));


        self.add_tag(AssetsKind::Audio, "sfx");
        self.add_tag(AssetsKind::Audio, "music");
        self.add_tag(AssetsKind::Audio, "volume");

        self.add_tag(AssetsKind::Texture2D, "aa");
        self.add_tag(AssetsKind::Texture2D, "pa");

        self.add_tag(AssetsKind::Font, "aa");
        self.add_tag(AssetsKind::Font, "pa");

        self.add_tag(AssetsKind::Texture2D, "px");
        self.add_tag(AssetsKind::Texture2D, "px_x");
        self.add_tag(AssetsKind::Texture2D, "px_y");

        self.add_tag(AssetsKind::Texture2D, "margin");
        self.add_tag(AssetsKind::Texture2D, "margin_x");
        self.add_tag(AssetsKind::Texture2D, "margin_y");


        //self.add_tag(AssetsKind::Texture2D, "px", u16); // for spritesheet ?
        //self.add_tag(AssetsKind::Texture2D, "margin", u16);


        self
    }
}



pub fn generate_game_asset(asset_path : &str, code_path : &str)
{
    let mut gen = AssetStructGenerator::new();

    gen.add_default_extensions_and_tag();

    gen.root = AssetPath::new(asset_path, &mut gen).expect("The root folder have an .ignore tag");

    let c = &mut gen.code;

    c.push_ln("///! This code was generated automatically by macro_game_asset");
    c.push_ln("///! DO NOT EDIT IT BECAUSE IT WILL BE ERASED");
    c.push_ln("///! Thank, Thomas Mewily");
    c.push_ln("");

    c.push_ln("use game_engine::context::*;");
    c.push_ln("use game_engine::wrapper::*;");
    c.push_ln("");

    gen.generate_all();
    //println!();
    //println!("{}", gen.output);

    fs::write(code_path, gen.code.output).expect(&format!("Asset : can't write the file at {}", code_path));
}

type FullPath = String;

#[derive(PartialEq, Debug, Default)]
pub enum TagsKind
{
    #[default]
    Add,
    Remove,
    Toggle,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Tag
{
    name : String,
    /// (arg...)
    argument : String,
}
impl Display for Tag
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        if self.is_credit()
        {
            write!(f, "credit(\"{}\")", self.argument)
        }else
        {
            write!(f, "{}{}", self.name, self.argument)
        }
    }
}

impl Tag
{
    pub fn new(value_and_arg : String) -> Self 
    {
        let idx = value_and_arg.char_indices().position(|(_, c)| c == '(');
        match idx
        {
            Some(split_index) => 
            {
                Self { name : value_and_arg[..split_index].to_owned(), argument : value_and_arg[split_index..].to_owned() }
            },
            None => 
            {
                Self { name : value_and_arg, argument : "()".to_owned() }
            },
        }
    }

    pub fn is_for_any_kind(&self) -> bool { self.is_credit() || self.is_ignore()}

    pub fn is_credit(&self) -> bool { self.name == "credits" || self.name == "credit" }
    pub fn is_ignore(&self) -> bool { self.name == "ignore" }
}

#[derive(PartialEq, Debug, Default)]
pub struct TagOrder
{
    tag  : Tag,
    kind : TagsKind
}
impl Deref for TagOrder {
    type Target=Tag;
    fn deref(&self) -> &Self::Target {
        &self.tag
    }
}
impl DerefMut for TagOrder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.tag
    }
}

impl TagOrder
{
    pub fn new(value : String) -> Self 
    { 
        match value.chars().next()
        {
            Some(c) => 
            {
                match c
                {
                    '+' => Self {tag : Tag::new(value.get(1..).unwrap().to_owned()), kind : TagsKind::Add},
                    '-' => Self {tag : Tag::new(value.get(1..).unwrap().to_owned()), kind : TagsKind::Remove},
                    '!' => Self {tag : Tag::new(value.get(1..).unwrap().to_owned()), kind : TagsKind::Toggle},
                    _   => Self {tag : Tag::new(value), kind : TagsKind::Add },
                }
            },
            None => Self::default(),
        }
    }
}

#[derive(Default)]
struct AssetPath
{
    /// the path (including the tag)
    full_path : FullPath,

    // the path name with the tags and extension
    //full_name : String,

    /// the path name before any dot. If a dir name is empty, it will be unpacked into it's parent
    name : String,

    /// For folder. Include an ID for uniqueness...
    struct_type_name : String,

    extension : AssetExtension,

    /// The tag collection (addditionnal extension : the `.<tag>` before final extension)
    /// 
    /// If the tags start by an
    /// `!` : toggle the flags
    /// `-` : remove the flags
    /// `+` or empty : add the flags
    tags : Vec<TagOrder>,

    /// The childs
    childs : Vec<Self>,
}

impl AssetPath
{
    pub fn is_dir(&self) -> bool { self.extension.is_dir()}

    pub fn new(path : &str, gen : &mut AssetStructGenerator) -> Option<Self>
    {
        let path = path.to_owned();

        let p = Path::new(&path);

        let full_name = if let Some(file_name) = p.file_name() {
            if let Some(file_name_str) = file_name.to_str() {
                file_name_str
            } else {
                panic!("File/Folder name \"{:?}\" is not valid UTF-8", file_name);
            }
        } else {
            panic!("Can't found file or folder : \"{}\"", path);
        };

        //println!("asset path : full name is '{}' for '{}'", full_name, path);

        let mut tags_unparsed : Vec<String> = full_name.split(".").map(|e| e.to_owned()).collect();
        let name = tags_unparsed.remove(0);
        let mut struct_type_name = Self::to_type_name(&name);

        let extension = match tags_unparsed.last()
        {
            Some(maybe_extension) => 
            {
                match gen.extensions_to_asset.get(maybe_extension) 
                {
                    Some(i_know_this_extension) => { tags_unparsed.pop(); i_know_this_extension.clone()},
                    None => AssetExtension::new_dir(),
                }
            },
            None => AssetExtension::new_dir(),
        };

        let tags : Vec<TagOrder> = tags_unparsed.into_iter().map(|t| TagOrder::new(t)).collect();
        if tags.iter().any(|e| e.is_ignore()) { return None; }

        let childs =
            if p.is_dir()
            {
                let id = gen.gen_id();
                if id != 0
                {
                    struct_type_name += &format!("Id{}", gen.gen_id());
                }
                let sub_dir : Vec<PathBuf> = fs::read_dir(p).expect("Can't read the asset folder").map(|p| p.unwrap().path()).collect();
                let child_path : Vec<&str> = sub_dir.iter().map(|e| e.as_path().as_os_str().to_str().unwrap()).collect();
                let childs = child_path.iter().filter_map(|path| Self::new(path, gen)).collect();
                childs
            }else
            {
                vec![]
            };

        //let full_name = full_name.to_owned();
        Some(Self { full_path: path, tags, name, extension, childs, struct_type_name })
    }

    fn type_name(&self) -> &String 
    {
        if self.is_dir()
        {
            &self.struct_type_name
            //self.to_type_name(&self.name)
        }else 
        { 
            &self.extension.game_engine_type_name
        }
    }
    
    fn to_type_name(s : &String) -> String 
    {
        if let Some(c) = s.chars().next() {
            let chars = s.chars();
            let first_char_uppercase = c.to_uppercase().to_string();
            let mut result = String::with_capacity(s.len());
            result.push_str(&first_char_uppercase);
            result.push_str(chars.as_str().get(1..).unwrap_or(""));
    
            format!("Asset{}", result)
        } else {
            String::new()
        }
    }

    /// empty dir are inlined into the parent
    pub fn is_empty_name_dir(&self) -> bool { self.is_dir() && self.name.is_empty() }

    pub fn emit_field(&self, code : &mut CodeEmitter)
    {
        if self.is_empty_name_dir()
        {
            code.push_and_write_ident(); 
            for c in self.childs.iter()
            {
                c.emit_field(code);
            }
            code.pop_ident();
            return;
        }
        code.push_ln(&format!("pub {} : {},", self.name, self.type_name()));
    }

    pub fn load_field(&self, code : &mut CodeEmitter, mut tags : Tags, supported_tag : &SupportedTagForExtension)
    {
        tags.apply_orders(&self.tags);

        if self.is_empty_name_dir()
        {
            code.push_and_write_ident(); 
            for c in self.childs.iter()
            {
                c.load_field(code, tags.clone(), supported_tag);
            }
            code.pop_ident();
            return;
        }

        if self.is_dir()
        {
            code.push_ln(&format!("{} : {}::load(a).await,", self.name, self.type_name()));
            return;
        }

        code.push(&format!("{} : a.load_{}(r\"{}\").await", self.name, self.type_name().to_lowercase(), self.full_path));
        
        tags.chain_call(code, supported_tag.get(&self.extension.kind).unwrap());

        code.push_char(',');
        code.ln();

    }


    pub fn get_credits(&self, code : &mut CodeEmitter)
    {
        if self.is_empty_name_dir()
        {
            code.push_and_write_ident(); 
            for c in self.childs.iter()
            {
                c.get_credits(code);
            }
            code.pop_ident();
            return;
        }

        if self.is_dir()
        {
            code.push(&format!("s.push(\"{} :\".to_owned()); ", self.name));
        }else
        {
            code.push(&format!("s.push(\"{} :\".to_owned()); ", self.name));

            //code.push(&format!("s.push(\"{}\"); ", self.name));
        }

        code.push_ln(&format!("self.{}.get_credits(s);", self.name));
        //code.push_ln(&format!("s.push(format!(\"{{}}\", self.{}.get_credit()));", self.name));
    }


    pub fn generate_code(&self, code : &mut CodeEmitter, tags : Tags, supported_tag : &SupportedTagForExtension)
    {
        if self.is_dir()
        {
            if self.is_empty_name_dir()
            {
                for c in self.childs.iter()
                {
                    c.generate_code(code, tags.clone(), supported_tag);
                }
                // will be unpacked into the parent
                return;
            }

            let type_name = self.type_name();

            code.push_ln(&format!("pub struct {} {{", type_name));
            code.push_and_write_ident();
            for c in self.childs.iter()
            {
                c.emit_field(code);
            }
            code.pop_ident();
            code.ln();
            code.push_ln("}");

            code.ln();
            code.ln();



            code.push_ln(&format!("impl {} {{", type_name));
            code.push_and_write_ident();
            {
                code.push_ln("pub async fn load(a : &mut ContextAssetManager) -> Self {");
                code.push_and_write_ident();
                {
                    code.push_ln("Self {");
                    code.push_and_write_ident();
                    {
                        let mut t = tags.clone();
                        t.apply_orders(&self.tags);
                        for c in self.childs.iter()
                        {
                            c.load_field(code, t.clone(), supported_tag);
                        }
                    }
                    code.pop_ident();
                    code.ln();
                    code.push_ln("}");
                }
                code.pop_ident();
                code.ln();
                code.push_ln("}");


                code.push_ln("pub fn get_credits(&self, s : &mut Vec<String>) {");
                code.push_and_write_ident();
                for c in self.childs.iter()
                {
                    c.get_credits(code);
                }
                code.ln();
                code.pop_ident();
                code.push_ln("}");
            }
            code.pop_ident();
            code.ln();
            code.push_ln("}");
            code.ln();
            code.ln();

            for c in self.childs.iter()
            {
                c.generate_code(code, tags.clone(), supported_tag);
            }
        }
    }
}


#[derive(Default, PartialEq, Clone, Debug)]
pub struct Tags
{
    tags : Vec<Tag>,
}
impl Tags
{
    pub fn new() -> Self { Self::default() }

    pub fn contains(&self, tag : &String) -> bool { self.tags.iter().position(|e| &e.name == tag).is_some() }
    pub fn remove_last(&mut self, tag : &String) 
    {
        self.tags.iter().position(|e| &e.name == tag).map(|idx| self.tags.remove(idx));
    }
    fn add(&mut self, tag : Tag) { self.tags.push(tag); } 

    pub fn apply_orders(&mut self, orders : &Vec<TagOrder>) { orders.iter().for_each(|o| self.apply_order(o)); }
    pub fn apply_order(&mut self, order : &TagOrder)
    {
        match order.kind
        {
            TagsKind::Add => { self.remove_last(&order.name); self.add(order.tag.clone()); },
            TagsKind::Remove => { self.remove_last(&order.name); },
            TagsKind::Toggle => 
            { 
                if self.contains(&order.name)
                {
                    self.remove_last(&order.name);
                }
                else
                {
                    self.add(order.tag.clone());
                }
            },
        }
    }

    fn chain_call(&self, code : &mut CodeEmitter, supported_tag : &SupportedTagName)
    {
        for t in self.tags.iter()
        {
            if supported_tag.contains(&t.name) || t.is_for_any_kind()
            {
                code.push(&format!(".tag_add_{}", t));
            }
        }
    }
}


type SupportedTagForExtension = HashMap<AssetsKind, SupportedTagName>;
type SupportedTagName = HashSet<String>;
type ExtensionToAsset = HashMap<String, AssetExtension>;
#[derive(Default)]
struct AssetStructGenerator
{
    root                : AssetPath,
    extensions_to_asset : ExtensionToAsset,
    supported_tag       : SupportedTagForExtension,
    code                : CodeEmitter,
    inc_nb              : u64,
}

#[derive(Default)]
struct CodeEmitter
{
    ident  : usize,
    output : String,
}
impl CodeEmitter
{
    pub fn write_ident(&mut self) 
    {
        for _ in 0..self.ident { self.write_one_ident(); }
    }

    pub fn write_one_ident(&mut self)
    {
        for _ in 0..3 { self.output.push(' '); }
    }

    pub fn push_ident(&mut self) { self.ident+=1; }
    pub fn pop_ident (&mut self) { self.ident-=1; }

    pub fn push_and_write_ident(&mut self) { self.push_ident(); self.write_one_ident(); }

    pub fn ln(&mut self) { self.push("\n"); self.write_ident(); }
    pub fn push_ln(&mut self, code_to_add : &str) { self.push(code_to_add); self.ln(); }
    pub fn push(&mut self, code_to_add : &str) { self.output.push_str(code_to_add.into()) }
    pub fn push_char(&mut self, ch : char) { self.output.push(ch); }
}

#[derive(Copy, Clone, Default, PartialEq, Eq, Hash, Debug)]
enum AssetsKind
{
    #[default]
    Directory,

    /// For textures such as .png
    Texture2D,
    Font,
    Audio,
}
impl AssetsKind
{
    pub fn is_dir(&self) -> bool { matches!(self, AssetsKind::Directory) }
}

#[allow(dead_code)]
#[derive(Clone, Default, Hash, PartialEq, Eq)]
struct AssetExtension
{
    /// the extension
    pub extension  : String,
    /// in `PascalCase`
    pub game_engine_type_name  : String,
    /// in `snake_case`
    pub game_engine_field_name : String,
    /// asset kind
    pub kind       : AssetsKind,
}
impl Deref for AssetExtension {
    type Target=AssetsKind;
    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}
impl DerefMut for AssetExtension
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.kind
    }
}

impl AssetStructGenerator
{
    pub fn gen_id(&mut self) -> u64 { self.inc_nb+=1; self.inc_nb-1 }

    pub fn add_tag(&mut self, extension : AssetsKind, tag : &str) 
    { 
        if !self.supported_tag.get_mut(&extension).unwrap().insert(tag.to_owned())
        {
            panic!("Tags `{}` is already defined for {:?}", tag, extension);
        }
    }

    pub fn add_extension(&mut self, extension_kind : AssetExtension) 
    { 
        if !self.supported_tag.contains_key(&extension_kind.kind)
        {
            self.supported_tag.insert(extension_kind.kind, HashSet::new());
        }
        match self.extensions_to_asset.insert(extension_kind.extension.clone(), extension_kind.clone())
        {
            Some(already_defined) => 
            {
                panic!("Asset extension `.{}` is already defined as a {}", extension_kind.extension, already_defined.game_engine_type_name);
            },
            None => {},
        }

    }
}

impl AssetExtension
{
    pub fn new_full(extension  : String, type_name : String, field_name : String, kind : AssetsKind) -> Self 
    { 
        Self { extension, game_engine_type_name: type_name, game_engine_field_name: field_name, kind }
    }
    pub fn new<Extension : Into<String>, Typename : Into<String>>(extension  : Extension, type_name : Typename, kind : AssetsKind) -> Self 
    { 
        let ex = extension.into();
        let tn = type_name.into();
        let tn_lower = tn.to_lowercase();
        Self::new_full(ex, tn, tn_lower, kind)
    }
    pub fn new_dir() -> Self { AssetExtension::new("", "", AssetsKind::Directory) }
}

impl AssetStructGenerator
{
    pub fn new() -> Self { Self::default() }

    pub fn generate_all(&mut self)
    {
        self.root.generate_code(&mut self.code, Tags::new(), &self.supported_tag);
    }
}
