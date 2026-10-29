interface ToolHeaderProps {
  title: string;
  description: string;
}

export function ToolHeader({ title, description }: ToolHeaderProps) {
  return (
    // Seluruh header jadi area geser jendela (judul jendela disembunyikan di macOS)
    <header data-tauri-drag-region className="shrink-0 px-6 pt-5 pb-4">
      <h1 data-tauri-drag-region className="text-xl font-semibold tracking-tight">
        {title}
      </h1>
      <p data-tauri-drag-region className="mt-1 text-sm text-muted-foreground">
        {description}
      </p>
    </header>
  );
}
