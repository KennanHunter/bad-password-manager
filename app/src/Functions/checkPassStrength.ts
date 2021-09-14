import zxcvbn from "zxcvbn";

export interface passwordReturnObject {
	score: number;
	text: string;
	warning: boolean;
}
export function checkPassStrength(password: string): passwordReturnObject {
	let textmap = ["Very Weak", "Weak", "Decent", "Strong"];
	let colormap = []; // TODO: Coloring of output
	let warningmap = [true, true, true, false];
	let data = zxcvbn(password);
	return {
		score: data.score,
		text: textmap[data.score],
		warning: warningmap[data.score],
	};
}
