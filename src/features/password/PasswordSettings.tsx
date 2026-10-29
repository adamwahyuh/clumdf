import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Eye, EyeOff } from "lucide-react";
import { Input } from "@/components/ui/input";
import { SettingRow } from "@/components/common/SettingRow";
import { SettingsPanel } from "@/components/common/SettingsPanel";

interface PasswordSettingsProps {
  password: string;
  onPasswordChange: (value: string) => void;
  /** Diisi untuk mode "kunci": minta password diketik ulang */
  confirm?: string;
  onConfirmChange?: (value: string) => void;
  hint: string;
}

/** Password hanya hidup di state layar; tidak pernah disimpan ke pengaturan. */
export function PasswordSettings({ password, onPasswordChange, confirm, onConfirmChange, hint }: PasswordSettingsProps) {
  const { t } = useTranslation();
  const [visible, setVisible] = useState(false);
  const mismatch = confirm !== undefined && confirm.length > 0 && confirm !== password;
  const type = visible ? "text" : "password";

  return (
    <SettingsPanel>
      <SettingRow label={t("password.label")} htmlFor="pdf-password" hint={hint}>
        <div className="relative">
          <Input
            id="pdf-password"
            type={type}
            value={password}
            onChange={(e) => onPasswordChange(e.target.value)}
            autoComplete="off"
            autoFocus
            className="pr-9"
          />
          <button
            type="button"
            onClick={() => setVisible((v) => !v)}
            aria-label={t(visible ? "password.hide" : "password.show")}
            className="absolute inset-y-0 right-0 flex w-9 items-center justify-center rounded-md text-muted-foreground hover:text-foreground focus-visible:ring-2 focus-visible:ring-ring focus-visible:outline-none"
          >
            {visible ? <EyeOff className="size-4" /> : <Eye className="size-4" />}
          </button>
        </div>
      </SettingRow>
      {onConfirmChange && (
        <SettingRow
          label={t("password.confirm")}
          htmlFor="pdf-password-confirm"
          hint={mismatch ? t("password.mismatch") : undefined}
        >
          <Input
            id="pdf-password-confirm"
            type={type}
            value={confirm}
            onChange={(e) => onConfirmChange(e.target.value)}
            autoComplete="off"
            aria-invalid={mismatch || undefined}
          />
        </SettingRow>
      )}
    </SettingsPanel>
  );
}
