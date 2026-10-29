import { useTranslation } from "react-i18next";
import type { StampPosition } from "@/lib/tauri/commands";
import { cn } from "@/lib/utils";

const rows: StampPosition[][] = [
  ["topLeft", "topCenter", "topRight"],
  ["bottomLeft", "bottomCenter", "bottomRight"],
];

interface PositionPickerProps {
  value: StampPosition;
  onChange: (position: StampPosition) => void;
  "aria-label": string;
}

/** Miniatur halaman dengan 6 titik posisi; berperilaku seperti radio group. */
export function PositionPicker({ value, onChange, "aria-label": ariaLabel }: PositionPickerProps) {
  const { t } = useTranslation();

  return (
    <div
      role="radiogroup"
      aria-label={ariaLabel}
      className="mx-auto flex aspect-[3/4] w-28 flex-col justify-between rounded-md bg-card p-2 shadow-card"
    >
      {rows.map((row) => (
        <div key={row[0]} className="grid grid-cols-3 gap-1">
          {row.map((position) => {
            const selected = position === value;
            return (
              <button
                key={position}
                type="button"
                role="radio"
                aria-checked={selected}
                aria-label={t(`positions.${position}`)}
                title={t(`positions.${position}`)}
                onClick={() => onChange(position)}
                className={cn(
                  "h-5 rounded transition-colors duration-150 focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none",
                  selected ? "bg-primary" : "bg-muted hover:bg-primary/30",
                )}
              />
            );
          })}
        </div>
      ))}
    </div>
  );
}
