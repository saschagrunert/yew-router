var N=null,E="",T="t",U="u",searchIndex={};
var R=["yew_router","result","try_into","try_from","borrow","type_id","typeid","borrow_mut","bridge","route","formatter","serialize","deserialize","handlerid","RouterAgent","Yew routing extension","A single route abstraction","path_segments","The URL path segments","The URL query part","fragment","The URL fragment part","The routes state","The RouterAgent worker holds on to the RouterService…","Request","The request to the RouterAgent","ChangeRoute","Changes the route using a RouteInfo struct and alerts…","GetCurrentRoute","Retrieve the current route request","to_route_string","Convert to a String","current_route","Retrieve the current route","routerservice","fallible","Convinience macro for route creation","to_owned","clone_into","callback","agentlink","connected","Add a client to the pool of connections of this agent","disconnected","Remove a client from the pool of connections of this agent"];
searchIndex[R[0]]={"doc":R[15],"i":[[3,"Route",R[0],R[16],N,N],[12,R[17],E,R[18],0,N],[12,"query",E,R[19],0,N],[12,R[20],E,R[21],0,N],[12,"state",E,R[22],0,N],[3,R[14],E,R[23],N,N],[4,R[24],E,R[25],N,N],[13,R[26],E,R[27],1,N],[13,R[28],E,R[29],1,N],[11,R[30],E,R[31],0,[[["self"]],["string"]]],[11,R[32],E,R[33],0,[[[R[34]]],[R[35]]]],[14,"routes",E,R[36],N,N],[11,"from",E,E,0,[[[T]],[T]]],[11,"into",E,E,0,[[],[U]]],[11,R[37],E,E,0,[[["self"]],[T]]],[11,R[38],E,E,0,[[[T],["self"]]]],[11,R[3],E,E,0,[[[U]],[R[1]]]],[11,R[4],E,E,0,[[["self"]],[T]]],[11,R[5],E,E,0,[[["self"]],[R[6]]]],[11,R[7],E,E,0,[[["self"]],[T]]],[11,R[2],E,E,0,[[],[R[1]]]],[11,R[2],E,E,0,[[],[R[1]]]],[11,"from",E,E,2,[[[T]],[T]]],[11,"into",E,E,2,[[],[U]]],[11,R[3],E,E,2,[[[U]],[R[1]]]],[11,R[4],E,E,2,[[["self"]],[T]]],[11,R[5],E,E,2,[[["self"]],[R[6]]]],[11,R[7],E,E,2,[[["self"]],[T]]],[11,R[2],E,E,2,[[],[R[1]]]],[11,R[2],E,E,2,[[],[R[1]]]],[11,R[8],E,E,2,[[[R[39]]],[[R[8]],["box",[R[8]]]]]],[11,"from",E,E,1,[[[T]],[T]]],[11,"into",E,E,1,[[],[U]]],[11,R[3],E,E,1,[[[U]],[R[1]]]],[11,R[4],E,E,1,[[["self"]],[T]]],[11,R[5],E,E,1,[[["self"]],[R[6]]]],[11,R[7],E,E,1,[[["self"]],[T]]],[11,R[2],E,E,1,[[],[R[1]]]],[11,R[2],E,E,1,[[],[R[1]]]],[11,"eq",E,E,0,[[["self"],[R[9]]],["bool"]]],[11,"ne",E,E,0,[[["self"],[R[9]]],["bool"]]],[11,"clone",E,E,0,[[["self"]],[R[9]]]],[11,"default",E,E,0,[[],[R[9]]]],[11,"fmt",E,E,0,[[[R[10]],["self"]],[R[1]]]],[11,"fmt",E,E,1,[[[R[10]],["self"]],[R[1]]]],[11,R[11],E,E,0,[[["__s"],["self"]],[R[1]]]],[11,R[11],E,E,1,[[["__s"],["self"]],[R[1]]]],[11,R[12],E,E,0,[[["__d"]],[R[1]]]],[11,R[12],E,E,1,[[["__d"]],[R[1]]]],[11,"create",E,E,2,[[[R[40]]],["self"]]],[11,"update",E,E,2,[[["self"]]]],[11,"handle",E,E,2,[[["self"],[R[13]]]]],[11,R[41],E,R[42],2,[[["self"],[R[13]]]]],[11,R[43],E,R[44],2,[[["self"],[R[13]]]]]],"p":[[3,"Route"],[4,R[24]],[3,R[14]]]};
searchIndex[R[0]]={"doc":R[15],"i":[[3,"Route",R[0],R[16],N,N],[12,R[17],E,R[18],0,N],[12,"query",E,R[19],0,N],[12,R[20],E,R[21],0,N],[12,"state",E,R[22],0,N],[3,R[14],E,R[23],N,N],[4,R[24],E,R[25],N,N],[13,R[26],E,R[27],1,N],[13,R[28],E,R[29],1,N],[11,R[30],E,R[31],0,[[["self"]],["string"]]],[11,R[32],E,R[33],0,[[[R[34]]],[R[35]]]],[14,"routes",E,R[36],N,N],[11,"from",E,E,0,[[[T]],[T]]],[11,"into",E,E,0,[[],[U]]],[11,R[37],E,E,0,[[["self"]],[T]]],[11,R[38],E,E,0,[[[T],["self"]]]],[11,R[3],E,E,0,[[[U]],[R[1]]]],[11,R[4],E,E,0,[[["self"]],[T]]],[11,R[5],E,E,0,[[["self"]],[R[6]]]],[11,R[7],E,E,0,[[["self"]],[T]]],[11,R[2],E,E,0,[[],[R[1]]]],[11,R[2],E,E,0,[[],[R[1]]]],[11,"from",E,E,2,[[[T]],[T]]],[11,"into",E,E,2,[[],[U]]],[11,R[3],E,E,2,[[[U]],[R[1]]]],[11,R[4],E,E,2,[[["self"]],[T]]],[11,R[5],E,E,2,[[["self"]],[R[6]]]],[11,R[7],E,E,2,[[["self"]],[T]]],[11,R[2],E,E,2,[[],[R[1]]]],[11,R[2],E,E,2,[[],[R[1]]]],[11,R[8],E,E,2,[[[R[39]]],[[R[8]],["box",[R[8]]]]]],[11,"from",E,E,1,[[[T]],[T]]],[11,"into",E,E,1,[[],[U]]],[11,R[3],E,E,1,[[[U]],[R[1]]]],[11,R[4],E,E,1,[[["self"]],[T]]],[11,R[5],E,E,1,[[["self"]],[R[6]]]],[11,R[7],E,E,1,[[["self"]],[T]]],[11,R[2],E,E,1,[[],[R[1]]]],[11,R[2],E,E,1,[[],[R[1]]]],[11,"eq",E,E,0,[[["self"],[R[9]]],["bool"]]],[11,"ne",E,E,0,[[["self"],[R[9]]],["bool"]]],[11,"clone",E,E,0,[[["self"]],[R[9]]]],[11,"default",E,E,0,[[],[R[9]]]],[11,"fmt",E,E,0,[[[R[10]],["self"]],[R[1]]]],[11,"fmt",E,E,1,[[[R[10]],["self"]],[R[1]]]],[11,R[11],E,E,0,[[["__s"],["self"]],[R[1]]]],[11,R[11],E,E,1,[[["__s"],["self"]],[R[1]]]],[11,R[12],E,E,0,[[["__d"]],[R[1]]]],[11,R[12],E,E,1,[[["__d"]],[R[1]]]],[11,"create",E,E,2,[[[R[40]]],["self"]]],[11,"update",E,E,2,[[["self"]]]],[11,"handle",E,E,2,[[["self"],[R[13]]]]],[11,R[41],E,R[42],2,[[["self"],[R[13]]]]],[11,R[43],E,R[44],2,[[["self"],[R[13]]]]]],"p":[[3,"Route"],[4,R[24]],[3,R[14]]]};
initSearch(searchIndex);addSearchOptions(searchIndex);