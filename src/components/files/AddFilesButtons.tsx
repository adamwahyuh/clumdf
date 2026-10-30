import { useTranslation } from "react-i18next";
import { FolderPlus, Plus } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";

interface AddFilesButtonsProps {
  onBrowse: () => void;
  onBrowseFolder: () => void;
}

/** "Tambah file" + tombol ikon "Tambah folder" untuk action bar. */
export function AddFilesButtons({ onBrowse, onBrowseFolder }: AddFilesButtonsProps) {
  const { t } = useTranslation();

  return (
    <>
      <Tooltip>
        <TooltipTrigger asChild>
          <Button variant="outline" size="icon" onClick={onBrowseFolder} aria-label={t("files.addFolder")}>
            <FolderPlus />
          </Button>
        </TooltipTrigger>
        <TooltipContent>{t("files.addFolder")}</TooltipContent>
      </Tooltip>
      <Button variant="outline" onClick={onBrowse}>
        <Plus />
        {t("files.add")}
      </Button>
    </>
  );
}
