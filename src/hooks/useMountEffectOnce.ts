import { useEffect, useRef } from "react";

export const useMountEffectOnce = (fn: () => void, deps: any[] = []) => {
	const wasExecutedRef = useRef(false);
	useEffect(() => {
		if (!wasExecutedRef.current) {
			fn();
		}
		wasExecutedRef.current = true;
		// eslint-disable-next-line react-hooks/exhaustive-deps
	}, [fn, ...deps]);
};
