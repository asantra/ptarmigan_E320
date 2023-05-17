/// Spacing (in log space) of the sample points in `LN_T_CHI_TABLE`.
pub const DELTA_LN_CHI: f64 = std::f64::consts::LN_10 / 20.0;

/// Table used for calculating the total pair creation rate.
/// Columns of log(chi), log(T_par(chi)), log(T_perp(chi)),
/// for sample points in the range 0.01 <= chi <= 100.
pub static LN_T_CHI_TABLE: [[f64; 3]; 81] = [
	[-4.605170185988091e+0, -2.685442596899664e+2, -2.678523509489123e+2],
	[-4.490040931338389e+0, -2.395446175365332e+2, -2.388528583490807e+2],
	[-4.374911676688687e+0, -2.136986840601857e+2, -2.130070922777788e+2],
	[-4.259782422038985e+0, -1.906634994081406e+2, -1.899720949604014e+2],
	[-4.144653167389282e+0, -1.701334006518743e+2, -1.694422057753670e+2],
	[-4.029523912739580e+0, -1.518359658244561e+2, -1.511450053133670e+2],
	[-3.914394658089878e+0, -1.355283990448902e+2, -1.348377005249106e+2],
	[-3.799265403440175e+0, -1.209943087621686e+2, -1.203039029887245e+2],
	[-3.684136148790473e+0, -1.080408363681679e+2, -1.073507575499281e+2],
	[-3.569006894140771e+0, -9.649609707767136e+1, -9.580638322596541e+1],
	[-3.453877639491069e+0, -8.620689911735530e+1, -8.551759242172274e+1],
	[-3.338748384841366e+0, -7.703671095851184e+1, -7.634785818807740e+1],
	[-3.223619130191664e+0, -6.886384961960987e+1, -6.817550254930436e+1],
	[-3.108489875541962e+0, -6.157986599817464e+1, -6.089208185798980e+1],
	[-2.993360620892259e+0, -5.508810580582737e+1, -5.440094775028451e+1],
	[-2.878231366242557e+0, -4.930242701035125e+1, -4.861596460416894e+1],
	[-2.763102111592855e+0, -4.414605676525564e+1, -4.346036647981513e+1],
	[-2.647972856943153e+0, -3.955057265804697e+1, -3.886573837168230e+1],
	[-2.532843602293450e+0, -3.545499475783937e+1, -3.477110825123795e+1],
	[-2.417714347643748e+0, -3.180497641293291e+1, -3.112213784905833e+1],
	[-2.302585092994046e+0, -2.855208305906011e+1, -2.787040143505358e+1],
	[-2.187455838344343e+0, -2.565314946653573e+1, -2.497274302309291e+1],
	[-2.072326583694641e+0, -2.306970689504003e+1, -2.239070346689354e+1],
	[-1.957197329044939e+0, -2.076747255203566e+1, -2.009000984152302e+1],
	[-1.842068074395237e+0, -1.871589457718435e+1, -1.804012033158623e+1],
	[-1.726938819745534e+0, -1.688774651157466e+1, -1.621381858412595e+1],
	[-1.611809565095832e+0, -1.525876586689714e+1, -1.458685214126238e+1],
	[-1.496680310446130e+0, -1.380733199464866e+1, -1.313761015336685e+1],
	[-1.381551055796427e+0, -1.251417897681281e+1, -1.184683609588526e+1],
	[-1.266421801146725e+0, -1.136213972420650e+1, -1.069737167871603e+1],
	[-1.151292546497023e+0, -1.033591788301040e+1, -9.673928552458963e+0],
	[-1.036163291847321e+0, -9.421884519445864e+0, -8.762884786346831e+0],
	[-9.210340371976183e-1, -8.607896882051377e+0, -7.952103423103596e+0],
	[-8.059047825479160e-1, -7.883136834932328e+0, -7.230770710641260e+0],
	[-6.907755278982137e-1, -7.237966817602181e+0, -6.589251873230981e+0],
	[-5.756462732485114e-1, -6.663801421054478e+0, -6.018962518950303e+0],
	[-4.605170185988091e-1, -6.152992878558481e+0, -5.512253988830993e+0],
	[-3.453877639491069e-1, -5.698728956057713e+0, -5.062311138896253e+0],
	[-2.302585092994046e-1, -5.294941893355101e+0, -4.663061211583222e+0],
	[-1.151292546497023e-1, -4.936227195598342e+0, -4.309092600051668e+0],
	[0.000000000000000e+0, -4.617771206800238e+0, -3.995582439523520e+0],
	[1.151292546497023e-1, -4.335286514898994e+0, -3.718232075835087e+0],
	[2.302585092994046e-1, -4.084954342663208e+0, -3.473209564403627e+0],
	[3.453877639491069e-1, -3.863373171906422e+0, -3.257098444253945e+0],
	[4.605170185988091e-1, -3.667512931200065e+0, -3.066852112925248e+0],
	[5.756462732485114e-1, -3.494674150656891e+0, -2.899753200164600e+0],
	[6.907755278982137e-1, -3.342451552400250e+0, -2.753377402374724e+0],
	[8.059047825479160e-1, -3.208701602954309e+0, -2.625561296773192e+0],
	[9.210340371976183e-1, -3.091513604825323e+0, -2.514373704985120e+0],
	[1.036163291847321e+0, -2.989183949758567e+0, -2.418090221081254e+0],
	[1.151292546497023e+0, -2.900193196243887e+0, -2.335170559545633e+0],
	[1.266421801146725e+0, -2.823185669428090e+0, -2.264238414884086e+0],
	[1.381551055796427e+0, -2.756951313231920e+0, -2.204063557063264e+0],
	[1.496680310446130e+0, -2.700409552654329e+0, -2.153545916127506e+0],
	[1.611809565095832e+0, -2.652594949407468e+0, -2.111701435545447e+0],
	[1.726938819745534e+0, -2.612644456535152e+0, -2.077649497406213e+0],
	[1.842068074395237e+0, -2.579786097846408e+0, -2.050601743788675e+0],
	[1.957197329044939e+0, -2.553328916118487e+0, -2.029852137702017e+0],
	[2.072326583694641e+0, -2.532654050324505e+0, -2.014768124147307e+0],
	[2.187455838344343e+0, -2.517206816818902e+0, -2.004782767258118e+0],
	[2.302585092994046e+0, -2.506489682639490e+0, -1.999387753304084e+0],
	[2.417714347643748e+0, -2.500056031004366e+0, -1.998127161728387e+0],
	[2.532843602293450e+0, -2.497504629821946e+0, -2.000591917469353e+0],
	[2.647972856943153e+0, -2.498474723703943e+0, -2.006414847706996e+0],
	[2.763102111592855e+0, -2.502641678672359e+0, -2.015266274987707e+0],
	[2.878231366242557e+0, -2.509713116570579e+0, -2.026850086516314e+0],
	[2.993360620892259e+0, -2.519425483204936e+0, -2.040900226359069e+0],
	[3.108489875541962e+0, -2.531541000529472e+0, -2.057177563461604e+0],
	[3.223619130191664e+0, -2.545844958809691e+0, -2.075467093834021e+0],
	[3.338748384841366e+0, -2.562143309722576e+0, -2.095575440066263e+0],
	[3.453877639491069e+0, -2.580260525826872e+0, -2.117328615580110e+0],
	[3.569006894140771e+0, -2.600037695822339e+0, -2.140570024762954e+0],
	[3.684136148790473e+0, -2.621330828557919e+0, -2.165158673420871e+0],
	[3.799265403440175e+0, -2.644009341891410e+0, -2.190967566886677e+0],
	[3.914394658089878e+0, -2.667954715288412e+0, -2.217882275669931e+0],
	[4.029523912739580e+0, -2.693059287513756e+0, -2.245799650782336e+0],
	[4.144653167389282e+0, -2.719225182948711e+0, -2.274626672851405e+0],
	[4.259782422038985e+0, -2.746363351993235e+0, -2.304279420880782e+0],
	[4.374911676688687e+0, -2.774392712712710e+0, -2.334682148056605e+0],
	[4.490040931338389e+0, -2.803239382388467e+0, -2.365766453361541e+0],
	[4.605170185988091e+0, -2.832835988953880e+0, -2.397470538963921e+0],
];
