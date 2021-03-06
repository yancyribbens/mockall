// vim: tw=80
use quote::{ToTokens, quote};
use syn::{
    *,
    spanned::Spanned
};

use crate::{
    mock_function::{self, MockFunction},
    compile_error
};

pub(crate) struct MockTrait {
    pub attrs: Vec<Attribute>,
    pub struct_generics: Generics,
    pub trait_generics: Generics,
    pub methods: Vec<MockFunction>,
    pub name: Ident,
    structname: Ident,
    pub types: Vec<TraitItemType>
}

impl MockTrait {
    pub fn name(&self) -> &Ident {
        &self.name
    }

    /// Create a new MockTrait
    ///
    /// # Arguments
    /// * `structname` - name of the struct that implements this trait
    /// * `struct_generics` - Generics of the parent structure
    /// * `trait_`  -    Mockable ItemTrait
    /// * `vis`     -   Visibility of the struct
    pub fn new(structname: &Ident,
               struct_generics: &Generics,
               trait_: ItemTrait,
               vis: &Visibility) -> Self
    {
        let mut methods = Vec::new();
        let mut types = Vec::new();
        for ti in trait_.items.into_iter() {
            match ti {
                TraitItem::Const(_) => {
                    // const items can easily be added by the user in a separate
                    // impl block
                },
                TraitItem::Method(tim) => {
                    let mf = mock_function::Builder::new(&tim.sig, &vis)
                        .attrs(&tim.attrs)
                        .levels(2)
                        .call_levels(0)
                        .struct_(structname)
                        .struct_generics(struct_generics)
                        .trait_(&trait_.ident)
                        .build();
                    methods.push(mf);
                },
                TraitItem::Type(tit) => {
                    types.push(tit);
                },
                _ => {
                    compile_error(ti.span(),
                    "This impl item is not yet supported by MockAll");
                }
            }
        }
        MockTrait {
            attrs: trait_.attrs,
            struct_generics: struct_generics.clone(),
            trait_generics: trait_.generics,
            methods,
            name: trait_.ident,
            structname: structname.clone(),
            types
        }
    }

    /// Generate code for the expect_ method
    ///
    /// # Arguments
    ///
    /// * `modname`:    Name of the parent struct's private module
    // Supplying modname is an unfortunately hack.  Ideally MockTrait
    // wouldn't need to know that.
    pub fn trait_impl(&self, modname: &Ident) -> impl ToTokens {
        let (ig, tg, wc) = self.struct_generics.split_for_impl();
        let (_, t_tg, _) = self.trait_generics.split_for_impl();
        let calls = self.methods.iter()
                .map(|meth| meth.call(Some(modname)))
                .collect::<Vec<_>>();
        let contexts = self.methods.iter()
            .filter(|meth| meth.is_static())
            .map(|meth| meth.context_fn(Some(modname)))
            .collect::<Vec<_>>();
        let expects = self.methods.iter()
            .filter(|meth| !meth.is_static())
            .map(|meth| meth.expect(&modname))
            .collect::<Vec<_>>();
        let name = &self.name;
        let structname = &self.structname;
        let types = &self.types;
        quote!(
            impl #ig #name #t_tg for #structname #tg #wc {
                #(#types)*
                #(#calls)*
            }
            impl #ig #structname #tg #wc {
                #(#expects)*
                #(#contexts)*
            }
        )
    }
}
