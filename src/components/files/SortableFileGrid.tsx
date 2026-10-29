import type { FileKind, QueuedFile } from "@/types/files";
import { FileCard } from "./FileCard";
import { SortableGrid } from "./SortableGrid";

interface SortableFileGridProps {
  files: QueuedFile[];
  kind: FileKind;
  onMove: (activeId: string, overId: string) => void;
  onRemove: (id: string) => void;
}

/** Grid file yang bisa diurutkan ulang; dipakai untuk PDF maupun gambar. */
export function SortableFileGrid({ files, kind, onMove, onRemove }: SortableFileGridProps) {
  return (
    <SortableGrid items={files} onMove={onMove}>
      {(file, index) => <FileCard key={file.id} file={file} index={index} kind={kind} onRemove={onRemove} />}
    </SortableGrid>
  );
}
