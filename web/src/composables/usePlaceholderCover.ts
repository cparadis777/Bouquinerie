

export function usePlaceholderCover(id: string): string{
  const code =id[0].charCodeAt(0);

  return `--placeholder-${(code % 5)+1}`
}
