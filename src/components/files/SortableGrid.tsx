import type { ReactNode } from "react";
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

interface SortableGridProps<T extends { id: string }> {
  items: T[];
  onMove: (activeId: string, overId: string) => void;
  /** Kartu tiap item; kartu wajib memakai `useSortable({ id: item.id })` */
  children: (item: T, index: number) => ReactNode;
}

/** Grid generik yang bisa diurutkan ulang dengan drag & drop atau keyboard. */
export function SortableGrid<T extends { id: string }>({ items, onMove, children }: SortableGridProps<T>) {
  const sensors = useSensors(
    // Jarak minimum supaya klik biasa (mis. tombol di kartu) tidak dianggap drag
    useSensor(PointerSensor, { activationConstraint: { distance: 4 } }),
    useSensor(KeyboardSensor, { coordinateGetter: sortableKeyboardCoordinates }),
  );

  function handleDragEnd({ active, over }: DragEndEvent) {
    if (over && active.id !== over.id) onMove(String(active.id), String(over.id));
  }

  return (
    <DndContext sensors={sensors} collisionDetection={closestCenter} onDragEnd={handleDragEnd}>
      <SortableContext items={items.map((item) => item.id)} strategy={rectSortingStrategy}>
        <div className="grid grid-cols-[repeat(auto-fill,minmax(148px,1fr))] gap-4">
          {items.map((item, index) => children(item, index))}
        </div>
      </SortableContext>
    </DndContext>
  );
}
