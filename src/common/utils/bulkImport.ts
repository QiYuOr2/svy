import path from 'path';
import glob from 'glob';
import { createAction } from './createAction';

export function bulkImport(dir: string): ReturnType<typeof createAction>[] {
  const files = glob.sync(path.join(dir, '*.ts'));
  return files.map((file) => {
    const key = /(?<=\/)(\w+)(?=\.)/.exec(file)![0];
    return require(file)[key];
  });
}
