import { useState, createContext } from "react";
import type { ReactNode } from "react";
import { useMountEffectOnce } from "../hooks/useMountEffectOnce";

const initial: IWASMContext = {};

export const WASMContext = createContext(initial);

export const WASMContextProvider: React.FC<WASMContextProviderProps> = ({
	children
}) => {
	const [state, setState] = useState<IWASMContext>(initial);

	// This has to run only once: https://github.com/rustwasm/wasm-bindgen/issues/3153
	// Though, in development React renders twice when Strict Mode is enabled: https://reactjs.org/docs/strict-mode.html
	// That's why it must be limited to a single mount run
	useMountEffectOnce(() => {
		(async () => {
			const wasm = await import("@/client/client");
			await wasm.default().catch(error => {
				if (
					!error.message.startsWith(
						"Using exceptions for control flow, don't mind me. This isn't actually an error!"
					)
				) {
					throw error;
				}
			});
			setState({ wasm });
		})();
	});

	return (
		<WASMContext.Provider value={state}>{children}</WASMContext.Provider>
	);
};

interface IWASMContext {
	wasm?: typeof import("@/client/client");
}

interface WASMContextProviderProps {
	children: ReactNode;
}
