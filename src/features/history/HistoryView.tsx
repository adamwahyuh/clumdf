import { useTranslation } from "react-i18next";
import { History, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { ToolHeader } from "@/components/layout/ToolHeader";
import { ActionBar } from "@/components/common/ActionBar";
import { useHistory } from "@/stores/history-store";
import { HistoryItem } from "./HistoryItem";

export function HistoryView() {
  const { t } = useTranslation();
  const { entries, remove, clear } = useHistory();

  return (
    <>
      <ToolHeader title={t("tools.history.title")} description={t("tools.history.description")} />
      <div className="min-h-0 flex-1 overflow-y-auto px-6 pb-6">
        {entries.length === 0 ? (
          <div className="flex h-full flex-col items-center justify-center gap-3 text-center">
            <span className="flex size-16 items-center justify-center rounded-2xl bg-muted">
              <History className="size-8 text-muted-foreground" strokeWidth={1.5} />
            </span>
            <p className="text-[15px] font-medium">{t("history.empty")}</p>
            <p className="max-w-sm text-sm text-muted-foreground">{t("history.emptyHint")}</p>
          </div>
        ) : (
          <ul className="flex flex-col gap-2">
            {entries.map((entry) => (
              <HistoryItem key={entry.id} entry={entry} onRemove={() => remove(entry.id)} />
            ))}
          </ul>
        )}
      </div>
      <ActionBar summary={t("history.summary", { count: entries.length })}>
        <Button variant="outline" disabled={!entries.length} onClick={clear}>
          <Trash2 />
          {t("history.clear")}
        </Button>
      </ActionBar>
    </>
  );
}
