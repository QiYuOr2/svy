import { createAction } from './createAction';

export function bulkImport(filenames: string[]): ReturnType<typeof createAction>[] {
  return filenames.map((file) => {
    const key = /(?<=\/)(\w+)(?=\.)/.exec(file)![0];
    return require(file)[key];
  });
}
