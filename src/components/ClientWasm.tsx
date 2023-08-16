"use client";

import { WASMContext } from "./ClientWasmContext";
import { useContext, useEffect, useRef } from "react";

export default function ClientWasm() {
	const wasm = useContext(WASMContext);

	// useEffect(() => {
	//     if (wasm.wasm) {
	//         // TODO: wasm
	//     }
	// }, [wasm.wasm])

	return (
		<div className="h-full w-full">
			<canvas id="canvas"></canvas>
		</div>
	);
}
