[
//  // hello :D
    SlashComment, Newlines,

//  pub fn a<T>(&mut self, lol: &u8, lmao: Vec<Vec<T>>) { //
    Pub, Spaces, Fn, Spaces, FnName, GenericStart, Type, GenericEnd, ParenStart,
    Reference, Mut, Spaces, SmallSelf, Comma, Spaces,
    VarName, Colon, Spaces, Reference, Type, Comma, Spaces,
    VarName, Colon, Spaces, Type, GenericStart, Type, GenericStart, Type, GenericEnd, GenericEnd,
    ParenEnd, Spaces, ScopeStart, Spaces, SlashComment, Newlines,

//  }
    ScopeEnd, Newlines,

//  fn b() {}
    Fn, Spaces, FnName, ParenStart, ParenEnd, Spaces, ScopeStart, ScopeEnd, Newlines
]
