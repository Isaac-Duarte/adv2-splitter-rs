var searchIndex = JSON.parse('{\
"adv2_splitter":{"doc":"","t":[3,17,0,11,11,0,12,11,11,14,11,14,14,14,14,14,14,14,0,5,5,5,12,12,5,5,11,11,11,5,5,5,5,8,3,13,13,13,7,4,13,3,3,7,7,7,13,4,13,13,13,7,13,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,10,11,11,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,11,12,11,11,11,11,11,11,11,11,12,0,11,12,11,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,12,12,12,12,12,12,12,12,12],"n":["AppArgs","HELP","advanced_dupe","borrow","borrow_mut","codec","file","fmt","from","get_lua_value","into","lua_angle","lua_array","lua_bool","lua_double","lua_string","lua_table","lua_vector","macro","main","parse_args","perform_split","print","size","split_lua_tables","split_map","try_from","try_into","type_id","decode","encode","print_result","print_value","AdCodec","AdCodec5","Angle","Array","Bool","CRLF","CodecError","Double","DuplicationInfo","DuplicationResult","HEADER","INFO_END","INFO_SPLIT","InvalidHeader","LuaValue","String","Table","UnsupportedCodec","VERSION","Vector","add_value","as_angle","as_angle_mut","as_array","as_array_mut","as_bool","as_double","as_double_mut","as_str","as_table","as_table_mut","as_vector","as_vector_mut","borrow","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","date","date","decode","decode","decode_stream","default","double_as_bytes","encode","encode","encode_stream","eq","fmt","fmt","fmt","from","from","from","from","from","get_duplication_info","hash","info","info","info_mut","into","into","into","into","into","is_angle","is_array","is_bool","is_double","is_string","is_table","is_valid_signature","is_valid_signature","is_vector","player_name","player_name","provide","read_double","set_date","set_player_name","set_size","set_time","set_time_zone","size","size","test","time","time","time_zone","time_zone","to_owned","to_owned","to_owned","to_string","try_from","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","type_id","value","value","value_mut","values","0","0","0","0","0","0","0","0"],"q":["adv2_splitter","","","","","","","","","","","","","","","","","","","","","","","","","","","","","adv2_splitter::advanced_dupe","","","","adv2_splitter::codec","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","adv2_splitter::codec::CodecError","adv2_splitter::codec::LuaValue","","","","","",""],"d":["","","","","","","","","Returns the argument unchanged.","Returns an optional of a <code>LuaValue</code> given the enum type","Calls <code>U::from(self)</code>.","","","","","Returns a LuaValue::String given a splice","","","","","","","","","Splits a Lua table into multiple tables using a provided …","Splits the given <code>map</code> into <code>n</code> chunks.","","","","","","","","","The implementation for the 5th codec version is as follows:","","","","","","","","","","","","","The LuaValue enum is used to represent supported values …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Returns a Duplication Result that will contain the parsed …","Decodes the <code>reader</code> into a <code>DuplicationResult</code> by reading the …","Decodes a stream of binary data into a <code>LuaValue</code> object.","","Returns serialized double represented as a byte array","Arguments","Encode the <code>DuplicationInfo</code> and <code>LuaValue</code> to a binary format.","Serializes a <code>LuaValue</code> into a binary stream.","","","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the duplication information that was stored in the …","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","Returns if the signature byte array is valid or not.","Returns true if <code>signature</code> is a valid header signature.","","","","","Returns deserialized double","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,0,1,1,0,1,1,1,0,1,0,0,0,0,0,0,0,0,0,0,0,1,1,0,0,1,1,1,0,0,0,0,0,0,8,8,8,0,0,8,0,0,0,0,0,22,0,8,8,22,0,8,15,8,8,8,8,8,8,8,8,8,8,8,8,14,15,8,24,22,14,15,8,24,22,15,8,22,15,8,22,15,15,28,24,24,24,24,28,24,24,8,8,22,22,14,15,8,24,22,24,8,14,14,14,14,15,8,24,22,8,8,8,8,8,8,28,24,8,15,15,22,24,15,15,15,15,15,15,15,0,15,15,15,15,15,8,22,22,14,15,8,24,22,14,15,8,24,22,14,15,8,24,22,14,14,14,15,29,30,31,32,33,34,35,36],"f":[0,0,0,[[]],[[]],0,0,[[1,2],3],[[]],0,[[]],0,0,0,0,0,0,0,0,[[],[[6,[[5,[4]]]]]],[[],[[6,[1,7]]]],[1,[[6,[[5,[4]]]]]],0,0,[[8,9],[[6,[[9,[8]],[5,[4]]]]]],[[10,11],[[9,[[10,[8,8]]]]]],[[],6],[[],6],[[],12],[13,[[6,[14,[5,[4]]]]]],[[15,8,16],[[6,[[5,[4]]]]]],[14],[[8,11]],0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,[[15,17,17]],[8,[[18,[9]]]],[8,[[18,[9]]]],[8,[[18,[9]]]],[8,[[18,[9]]]],[8,[[18,[19]]]],[8,[[18,[20]]]],[8,[[18,[20]]]],[8,[[18,[21]]]],[8,[[18,[10]]]],[8,[[18,[10]]]],[8,[[18,[9]]]],[8,[[18,[9]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[15,15],[8,8],[22,22],[[]],[[]],[[]],[15,21],0,[23,14],[[24,23],14],[25,[[18,[[5,[8]]]]]],[[],24],[20],[[15,8,16],[[6,[[5,[4]]]]]],[[24,15,8,16],[[6,[[5,[4]]]]]],[8,[[6,[[5,[4]]]]]],[[8,8],19],[[8,2],3],[[22,2],3],[[22,2],3],[[]],[[]],[[]],[[]],[[]],[[],15],[8],[14,15],0,[14,15],[[]],[[]],[[]],[[]],[[]],[8,19],[8,19],[8,19],[8,19],[8,19],[8,19],[[],19],[24,19],[8,19],[15,21],0,[26],[23,20],[[15,17]],[[15,17]],[[15,27]],[[15,17]],[[15,17]],[15,27],0,0,[15,21],0,[15,21],0,[[]],[[]],[[]],[[],17],[[],6],[[],6],[[],6],[[],6],[[],6],[[],6],[[],6],[[],6],[[],6],[[],6],[[],12],[[],12],[[],12],[[],12],[[],12],[14,8],0,[14,8],0,0,0,0,0,0,0,0,0],"p":[[3,"AppArgs"],[3,"Formatter"],[6,"Result"],[8,"Error"],[3,"Box"],[4,"Result"],[4,"Error"],[4,"LuaValue"],[3,"Vec"],[3,"HashMap"],[15,"usize"],[3,"TypeId"],[3,"File"],[3,"DuplicationResult"],[3,"DuplicationInfo"],[8,"Write"],[3,"String"],[4,"Option"],[15,"bool"],[15,"f64"],[15,"str"],[4,"CodecError"],[8,"Read"],[3,"AdCodec5"],[3,"Cursor"],[3,"Demand"],[15,"u32"],[8,"AdCodec"],[13,"UnsupportedCodec"],[13,"String"],[13,"Array"],[13,"Table"],[13,"Double"],[13,"Vector"],[13,"Angle"],[13,"Bool"]]}\
}');
if (typeof window !== 'undefined' && window.initSearch) {window.initSearch(searchIndex)};
if (typeof exports !== 'undefined') {exports.searchIndex = searchIndex};