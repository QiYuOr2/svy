import fs from 'fs-extra';
import { SVYRC } from '../constants';

function initConfig() {
  const config = {
    registryList: {
      npm: 'https://registry.npmjs.org/',
      taobao: 'https://registry.npmmirror.com/',
    },
  };
  fs.writeFileSync(SVYRC, JSON.stringify(config));
  return config;
}

export function checkConfig(): Record<string, any> {
  if (fs.existsSync(SVYRC)) {
    return JSON.parse(fs.readFileSync(SVYRC, 'utf-8'));
  }
  return initConfig();
}

export function updateConfig(key: string, value: any) {
  const config = checkConfig();
  config[key] = value;

  fs.writeFileSync(SVYRC, JSON.stringify(config));

  return [config[key], config];
}
