import { useTranslation } from "react-i18next";
import { CheckCheck, RotateCcw, RotateCw, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";

interface OrganizeToolbarProps {
  selectedCount: number;
  allSelected: boolean;
  onToggleAll: () => void;
  onRotate: (delta: number) => void;
  onRemove: () => void;
}

/** Aksi untuk halaman terpilih. */
export function OrganizeToolbar({ selectedCount, allSelected, onToggleAll, onRotate, onRemove }: OrganizeToolbarProps) {
  const { t } = useTranslation();
  const none = selectedCount === 0;

  return (
    <div className="sticky top-0 z-20 -mx-6 mb-4 flex flex-wrap items-center gap-1 border-b bg-background/90 px-6 py-2 backdrop-blur">
      <Button variant="ghost" size="sm" onClick={onToggleAll}>
        <CheckCheck />
        {t(allSelected ? "tools.organize.selectNone" : "tools.organize.selectAll")}
      </Button>
      <span className="mx-1 h-5 w-px bg-border" />
      <Button variant="ghost" size="sm" disabled={none} onClick={() => onRotate(-90)}>
        <RotateCcw />
        {t("tools.organize.rotateLeft")}
      </Button>
      <Button variant="ghost" size="sm" disabled={none} onClick={() => onRotate(90)}>
        <RotateCw />
        {t("tools.organize.rotateRight")}
      </Button>
      <Button variant="ghost" size="sm" disabled={none} onClick={onRemove} className="text-destructive hover:text-destructive">
        <Trash2 />
        {t("tools.organize.delete")}
      </Button>
      <span className="ml-auto text-xs text-muted-foreground">{t("tools.organize.hint")}</span>
    </div>
  );
}
