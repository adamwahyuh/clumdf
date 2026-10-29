import { useState } from "react";
import { useTranslation } from "react-i18next";
import { Lock } from "lucide-react";
import { BatchToolView } from "@/components/layout/BatchToolView";
import { useBatch } from "@/hooks/use-batch";
import { withSuffix } from "@/lib/format";
import { protectPdf, type PasswordReport } from "@/lib/tauri/commands";
import { createFileQueue, inspectPdfFile } from "@/stores/create-file-queue";
import { PasswordSettings } from "./PasswordSettings";

const useProtectQueue = createFileQueue(inspectPdfFile);

export function ProtectView() {
  const { t } = useTranslation();
  const [password, setPassword] = useState("");
  const [confirm, setConfirm] = useState("");

  const batch = useBatch<PasswordReport>(
    useProtectQueue,
    async (file) => {
      const output = withSuffix(file.path, "-protected");
      return { output, report: await protectPdf({ input: file.path, output, password }) };
    },
    password.length > 0 && password === confirm,
  );

  return (
    <BatchToolView
      i18nKey="tools.protect"
      icon={Lock}
      queue={useProtectQueue}
      batch={batch}
      settings={
        <PasswordSettings
          password={password}
          onPasswordChange={setPassword}
          confirm={confirm}
          onConfirmChange={setConfirm}
          hint={t("tools.protect.passwordHint")}
        />
      }
    />
  );
}
