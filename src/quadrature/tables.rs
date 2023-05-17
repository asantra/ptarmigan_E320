//! Nodes and weights used in Gauss-Legendre (finite interval) and Gauss-Laguerre (semi-infinite) quadrature

pub static GAUSS_16_NODES: [f64; 16] = [
    -9.894009349916499e-1,
    -9.445750230732326e-1,
    -8.656312023878317e-1,
    -7.554044083550030e-1,
    -6.178762444026437e-1,
    -4.580167776572274e-1,
    -2.816035507792589e-1,
    -9.501250983763744e-2,
    9.501250983763744e-2,
    2.816035507792589e-1,
    4.580167776572274e-1,
    6.178762444026437e-1,
    7.554044083550030e-1,
    8.656312023878317e-1,
    9.445750230732326e-1,
    9.894009349916499e-1,
];

pub static GAUSS_16_WEIGHTS: [f64; 16] = [
    2.715245941175400e-2,
    6.225352393864800e-2,
    9.515851168249300e-2,
    1.246289712555340e-1,
    1.495959888165770e-1,
    1.691565193950025e-1,
    1.826034150449236e-1,
    1.894506104550685e-1,
    1.894506104550685e-1,
    1.826034150449236e-1,
    1.691565193950025e-1,
    1.495959888165770e-1,
    1.246289712555340e-1,
    9.515851168249300e-2,
    6.225352393864800e-2,
    2.715245941175400e-2,
];

pub static GAUSS_32_NODES: [f64; 32] = [
    -9.972638618494816e-1,
    -9.856115115452683e-1,
    -9.647622555875064e-1,
    -9.349060759377397e-1,
    -8.963211557660521e-1,
    -8.493676137325700e-1,
    -7.944837959679424e-1,
    -7.321821187402897e-1,
    -6.630442669302152e-1,
    -5.877157572407623e-1,
    -5.068999089322294e-1,
    -4.213512761306353e-1,
    -3.318686022821276e-1,
    -2.392873622521371e-1,
    -1.444719615827965e-1,
    -4.830766568773832e-2,
    4.830766568773832e-2,
    1.444719615827965e-1,
    2.392873622521371e-1,
    3.318686022821276e-1,
    4.213512761306353e-1,
    5.068999089322294e-1,
    5.877157572407623e-1,
    6.630442669302152e-1,
    7.321821187402897e-1,
    7.944837959679424e-1,
    8.493676137325700e-1,
    8.963211557660521e-1,
    9.349060759377397e-1,
    9.647622555875064e-1,
    9.856115115452683e-1,
    9.972638618494816e-1,
];

pub static GAUSS_32_WEIGHTS: [f64; 32] = [
    7.018610000000000e-3,
    1.627439500000000e-2,
    2.539206500000000e-2,
    3.427386300000000e-2,
    4.283589800000000e-2,
    5.099805900000000e-2,
    5.868409350000000e-2,
    6.582222280000000e-2,
    7.234579411000000e-2,
    7.819389578700000e-2,
    8.331192422690000e-2,
    8.765209300440000e-2,
    9.117387869576400e-2,
    9.384439908080460e-2,
    9.563872007927486e-2,
    9.654008851472780e-2,
    9.654008851472780e-2,
    9.563872007927486e-2,
    9.384439908080460e-2,
    9.117387869576400e-2,
    8.765209300440000e-2,
    8.331192422690000e-2,
    7.819389578700000e-2,
    7.234579411000000e-2,
    6.582222280000000e-2,
    5.868409350000000e-2,
    5.099805900000000e-2,
    4.283589800000000e-2,
    3.427386300000000e-2,
    2.539206500000000e-2,
    1.627439500000000e-2,
    7.018610000000000e-3,
];

pub static GL_NODES: [f64; 32] = [
	4.448936583326702e-2,
	2.345261095196185e-1,
	5.768846293018864e-1,
	1.072448753817818e+0,
	1.722408776444645e+0,
	2.528336706425795e+0,
	3.492213273021994e+0,
	4.616456769749767e+0,
	5.903958504174244e+0,
	7.358126733186241e+0,
	8.982940924212596e+0,
	1.078301863253997e+1,
	1.276369798674273e+1,
	1.493113975552256e+1,
	1.729245433671531e+1,
	1.985586094033605e+1,
	2.263088901319677e+1,
	2.562863602245925e+1,
	2.886210181632347e+1,
	3.234662915396474e+1,
	3.610049480575197e+1,
	4.014571977153944e+1,
	4.450920799575494e+1,
	4.922439498730864e+1,
	5.433372133339691e+1,
	5.989250916213402e+1,
	6.597537728793505e+1,
	7.268762809066271e+1,
	8.018744697791352e+1,
	8.873534041789240e+1,
	9.882954286828397e+1,
	1.117513980979377e+2,
];

pub static GL_WEIGHTS: [f64; 32] = [
	1.092183419523850e-1,
	2.104431079388132e-1,
	2.352132296698480e-1,
	1.959033359728810e-1,
	1.299837862860718e-1,
	7.057862386571744e-2,
	3.176091250917507e-2,
	1.191821483483856e-2,
	3.738816294611525e-3,
	9.808033066149551e-4,
	2.148649188013642e-4,
	3.920341967987947e-5,
	5.934541612868633e-6,
	7.416404578667552e-7,
	7.604567879120781e-8,
	6.350602226625807e-9,
	4.281382971040929e-10,
	2.305899491891336e-11,
	9.799379288727094e-13,
	3.237801657729266e-14,
	8.171823443420719e-16,
	1.542133833393823e-17,
	2.119792290163619e-19,
	2.054429673788045e-21,
	1.346982586637395e-23,
	5.661294130397359e-26,
	1.418560545463037e-28,
	1.913375494454224e-31,
	1.192248760098222e-34,
	2.671511219240137e-38,
	1.338616942106256e-42,
	4.510536193898974e-48,
];

#[cfg(test)]
pub static CLENSHAW_CURTIS_15_NODES_WEIGHTS: [(f64, f64); 15] = [
    (0.00000000000e+0, 2.56410256410e-3),
    (1.25360439091e-2, 2.43496936475e-2),
    (4.95155660488e-2, 4.89101958380e-2),
    (1.09084258766e-1, 6.98325392478e-2),
    (1.88255099071e-1, 8.78028945005e-2),
    (2.83058130441e-1, 1.01025733741e-1),
    (3.88739533022e-1, 1.09440755815e-1),
    (5.00000000000e-1, 1.12148169291e-1),
    (6.11260466978e-1, 1.09440755815e-1),
    (7.16941869559e-1, 1.01025733741e-1),
    (8.11744900929e-1, 8.78028945005e-2),
    (8.90915741234e-1, 6.98325392478e-2),
    (9.50484433951e-1, 4.89101958380e-2),
    (9.87463956091e-1, 2.43496936475e-2),
    (1.00000000000e+0, 2.56410256410e-3),
];
