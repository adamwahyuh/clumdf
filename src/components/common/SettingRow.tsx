import type { ReactNode } from "react";
import { cn } from "@/lib/utils";

interface SettingRowProps {
  label: string;
  /** id kontrol supaya klik label memfokuskan kontrolnya */
  htmlFor?: string;
  hint?: string;
  /** true: label di kiri, kontrol (mis. Switch) di kanan */
  inline?: boolean;
  children: ReactNode;
}

export function SettingRow({ label, htmlFor, hint, inline, children }: SettingRowProps) {
  return (
    <div className="flex flex-col gap-1.5">
      <div className={cn("flex gap-2", inline ? "items-center justify-between" : "flex-col")}>
        <label htmlFor={htmlFor} className="text-[13px] font-medium">
          {label}
        </label>
        {children}
      </div>
      {hint && <p className="text-xs text-muted-foreground">{hint}</p>}
    </div>
  );
}
