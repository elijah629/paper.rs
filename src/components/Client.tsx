"use client";

import { WASMContextProvider } from "./ClientWasmContext";
import ClientWasm from "./ClientWasm";

export default function Client() {
	return (
		<WASMContextProvider>
			<ClientWasm />
		</WASMContextProvider>
	);
}
