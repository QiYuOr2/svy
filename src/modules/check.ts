import chalk from 'chalk';
import fs from 'fs-extra';
import path from 'path';
import { createAction, isUndefined } from '../common/utils';

const shouldIgnore = (dir: string) => {
  if (dir.slice(0, 1) === '.') {
    return true;
  }
  if (dir.indexOf('node_modules') !== -1) {
    return true;
  }
  return false;
};

/**
 * 递归读取当前目录下所有文件（包含多级目录）
 */
const readAllFile = (dir: string, result: string[] = []) => {
  if (!fs.statSync(dir).isDirectory()) {
    result.push(dir);
    return result;
  }

  const pathList = fs.readdirSync(dir);
  pathList.forEach(
    (p) => !shouldIgnore(p) && readAllFile(path.join(dir, p), result)
  );
  return result;
};

export const check = createAction({
  name: 'check',
  args: { dir: { type: String } },
  description: '检测目录中是否存在敏感信息',
  action({ dir }) {
    let shouldWarn = false;
    const currentDir = isUndefined(dir) ? process.cwd() : dir;
    readAllFile(currentDir).forEach((filePath) => {
      const fileContent = fs.readFileSync(filePath, { encoding: 'utf-8' });

      if (fileContent.match(/vdian|weidian/g)) {
        shouldWarn = true;
        console.log(
          chalk.red(
            `[${filePath.replace(
              currentDir,
              ''
            )}]中疑似含有敏感数据，请检查文件内容`
          )
        );
      }
    });

    !shouldWarn && console.log(chalk.green('检测通过!'));
  },
});
