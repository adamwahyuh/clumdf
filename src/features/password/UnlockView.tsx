import { useState } from "react";
import { useTranslation } from "react-i18next";
import { LockOpen } from "lucide-react";
import { BatchToolView } from "@/components/layout/BatchToolView";
import { useBatch } from "@/hooks/use-batch";
import { withSuffix } from "@/lib/format";
import { unlockPdf, type PasswordReport } from "@/lib/tauri/commands";
import { createFileQueue } from "@/stores/create-file-queue";
import { PasswordSettings } from "./PasswordSettings";

// Tanpa inspeksi: PDF terkunci justru yang diharapkan di sini, jangan ditandai error
const useUnlockQueue = createFileQueue();

export function UnlockView() {
  const { t } = useTranslation();
  const [password, setPassword] = useState("");

  const batch = useBatch<PasswordReport>(
    useUnlockQueue,
    async (file) => {
      const output = withSuffix(file.path, "-unlocked");
      return { output, report: await unlockPdf({ input: file.path, output, password }) };
    },
    password.length > 0,
  );

  return (
    <BatchToolView
      i18nKey="tools.unlock"
      icon={LockOpen}
      queue={useUnlockQueue}
      batch={batch}
      settings={<PasswordSettings password={password} onPasswordChange={setPassword} hint={t("tools.unlock.passwordHint")} />}
    />
  );
}
