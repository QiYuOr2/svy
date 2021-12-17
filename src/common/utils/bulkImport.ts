import { createAction } from './createAction';

export function transferName(name: string): string {
  if (name.indexOf('-') === -1) {
    return name;
  }
  const lineIndex = name.indexOf('-');
  const nextChar = name.slice(lineIndex + 1, lineIndex + 2);
  return transferName(name.replace(`-${nextChar}`, nextChar.toUpperCase()));
}

export function bulkImport(filenames: string[]): ReturnType<typeof createAction>[] {
  return filenames.map((file) => {
    const key = /(?<=\/)(\w+-?\w+)(?=\.)/.exec(file)![0];

    return require(file)[transferName(key)];
  });
}
