import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Plus, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { SettingRow } from "./SettingRow";

export interface PresetOption<T> {
  id: string;
  label: string;
  /** Hanya kunci yang disebut yang diterapkan & dibandingkan */
  values: Partial<T>;
}

interface PresetPickerProps<T> {
  builtIn: PresetOption<T>[];
  user: PresetOption<T>[];
  current: T;
  onApply: (values: Partial<T>) => void;
  onSave: (name: string) => void;
  onDelete: (id: string) => void;
}

const CUSTOM = "__custom";

function matches<T>(values: Partial<T>, current: T): boolean {
  return (Object.keys(values) as (keyof T)[]).every((key) => values[key] === current[key]);
}

/** Pilih preset bawaan/buatan sendiri, atau simpan pengaturan saat ini sebagai preset. */
export function PresetPicker<T>({ builtIn, user, current, onApply, onSave, onDelete }: PresetPickerProps<T>) {
  const { t } = useTranslation();
  const [naming, setNaming] = useState(false);
  const [name, setName] = useState("");

  const all = [...user, ...builtIn];
  // Preset buatan sendiri didahulukan bila nilainya kebetulan sama dengan bawaan
  const active = all.find((preset) => matches(preset.values, current));
  const activeIsUser = active && user.includes(active);

  function save() {
    if (!name.trim()) return;
    onSave(name.trim());
    setName("");
    setNaming(false);
  }

  return (
    <div className="flex flex-col gap-2 border-b pb-5">
      <SettingRow label={t("presets.label")} htmlFor="preset-picker">
        <Select
          value={active?.id ?? CUSTOM}
          onValueChange={(id) => {
            const preset = all.find((p) => p.id === id);
            if (preset) onApply(preset.values);
          }}
        >
          <SelectTrigger id="preset-picker" className="w-full">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value={CUSTOM} disabled>
              {t("presets.custom")}
            </SelectItem>
            {user.length > 0 && (
              <SelectGroup>
                <SelectLabel>{t("presets.mine")}</SelectLabel>
                {user.map((p) => (
                  <SelectItem key={p.id} value={p.id}>
                    {p.label}
                  </SelectItem>
                ))}
              </SelectGroup>
            )}
            <SelectGroup>
              <SelectLabel>{t("presets.builtIn")}</SelectLabel>
              {builtIn.map((p) => (
                <SelectItem key={p.id} value={p.id}>
                  {p.label}
                </SelectItem>
              ))}
            </SelectGroup>
          </SelectContent>
        </Select>
      </SettingRow>

      {naming ? (
        <form
          className="flex gap-1.5"
          onSubmit={(e) => {
            e.preventDefault();
            save();
          }}
        >
          <Input
            autoFocus
            value={name}
            onChange={(e) => setName(e.target.value)}
            onKeyDown={(e) => e.key === "Escape" && setNaming(false)}
            placeholder={t("presets.namePlaceholder")}
            aria-label={t("presets.nameLabel")}
            className="h-8"
          />
          <Button type="submit" size="sm" disabled={!name.trim()}>
            {t("presets.confirm")}
          </Button>
        </form>
      ) : (
        <div className="flex gap-1">
          <Button variant="ghost" size="sm" onClick={() => setNaming(true)} disabled={!!active}>
            <Plus />
            {t("presets.save")}
          </Button>
          {activeIsUser && (
            <Button variant="ghost" size="sm" className="text-destructive hover:text-destructive" onClick={() => onDelete(active.id)}>
              <Trash2 />
              {t("presets.delete")}
            </Button>
          )}
        </div>
      )}
    </div>
  );
}
