import { useDisplaySettings } from "@app/providers/DisplaySettingProvider";

export function DecimalSettings() {
  const { decimals, setDecimals } =
    useDisplaySettings();

  const options = [0, 1, 2, 3, 4, 5, 6] as const;

  return (
    <>
      {options.map((value) => (
        <button
          key={value}
          className={
            decimals === value ? "active" : ""
          }
          onClick={() => setDecimals(value)}
        >
          {value} desimal
          {value === 1 ? "" : "er"}
        </button>
      ))}

      <p
        style={{
          opacity: 0.6,
          marginTop: 8,
        }}
      >
        Nåværende: {decimals} desimal
        {decimals === 1 ? "" : "er"}
      </p>
    </>
  );
}
