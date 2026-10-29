import {
  closestCenter,
  DndContext,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
  type DragEndEvent,
} from "@dnd-kit/core";
import { rectSortingStrategy, SortableContext, sortableKeyboardCoordinates } from "@dnd-kit/sortable";
import type { FileKind, QueuedFile } from "@/types/files";
import { FileCard } from "./FileCard";

interface SortableFileGridProps {
  files: QueuedFile[];
  kind: FileKind;
  onMove: (activeId: string, overId: string) => void;
  onRemove: (id: string) => void;
}

/** Grid generik yang bisa diurutkan ulang; dipakai untuk PDF maupun gambar. */
export function SortableFileGrid({ files, kind, onMove, onRemove }: SortableFileGridProps) {
  const sensors = useSensors(
    // Jarak minimum supaya klik biasa (mis. tombol hapus) tidak dianggap drag
    useSensor(PointerSensor, { activationConstraint: { distance: 4 } }),
    useSensor(KeyboardSensor, { coordinateGetter: sortableKeyboardCoordinates }),
  );

  function handleDragEnd({ active, over }: DragEndEvent) {
    if (over && active.id !== over.id) onMove(String(active.id), String(over.id));
  }

  return (
    <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
      <SortableContext items={files.map((f) => f.id)} strategy={rectSortingStrategy}>
        <div className="grid grid-cols-[repeat(auto-fill,minmax(148px,1fr))] gap-4">
          {files.map((file, index) => (
            <FileCard key={file.id} file={file} index={index} kind={kind} onRemove={onRemove} />
          ))}
        </div>
      </SortableContext>
    </DndContext>
  );
}
