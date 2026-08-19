import { useEffect, useRef } from "react";

type UseAutoCalculateOptions<TForm> = {
  enabled: boolean;
  ready?: boolean;
  signature: string;
  form: TForm;
  calculate: (form: TForm) => Promise<TForm>;
  onResult: (form: TForm) => void;
  debounceMs?: number;
};

export function useAutoCalculate<TForm>({
  enabled,
  ready = true,
  signature,
  form,
  calculate,
  onResult,
  debounceMs = 250,
}: UseAutoCalculateOptions<TForm>) {
  const latestRef = useRef({ form, calculate, onResult });
  const requestIdRef = useRef(0);

  useEffect(() => {
    latestRef.current = { form, calculate, onResult };
  }, [form, calculate, onResult]);

  useEffect(() => {
    if (!enabled || !ready || !signature) return;

    const requestId = requestIdRef.current + 1;
    requestIdRef.current = requestId;

    const timeoutId = window.setTimeout(() => {
      const latest = latestRef.current;

      void latest
        .calculate(latest.form)
        .then((next) => {
          if (requestIdRef.current !== requestId) return;
          latestRef.current.onResult(next);
        })
        .catch((error: unknown) => {
          if (requestIdRef.current !== requestId) return;
          console.error(error);
        });
    }, debounceMs);

    return () => {
      window.clearTimeout(timeoutId);
      requestIdRef.current += 1;
    };
  }, [debounceMs, enabled, ready, signature]);
}
