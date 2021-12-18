import chalk from 'chalk';
import { execSync } from 'child_process';
import {
  checkConfig,
  createAction,
  isUndefined,
  updateConfig,
} from '../common/utils';

const getGitConfigByKey = (key: string) => {
  return checkConfig().gitConfigList[key];
};

const setGitConfig = (
  name: string,
  email: string,
  isGlobal: boolean = false
) => {
  try {
    execSync(`git config ${isGlobal ? '--global' : ''} user.name "${name}"`);
    execSync(`git config ${isGlobal ? '--global' : ''} user.email "${email}"`);

    console.log(chalk.green('设置完成\n'));
    actionMap.get();
  } catch (error) {
    console.log(chalk.red(JSON.stringify(error)));
  }
};

const actionMap = {
  get: () => {
    const getConfig = (scope?: string) => {
      scope = scope === 'global' ? '--global' : '';
      const username = execSync(`git config ${scope} --get user.name`)
        .toString('utf-8')
        .trim();
      const email = execSync(`git config ${scope} --get user.email`)
        .toString('utf-8')
        .trim();
      console.log(`user.name:\t${username}\nuser.email:\t${email}`);
    };

    console.log('全局配置:');
    getConfig('global');
    console.log(`========================================`);
    console.log('当前仓库配置:');
    getConfig();
  },

  preset: () => {
    const gitConfigList = checkConfig().gitConfigList;
    console.log('预设的git信息:\n');
    Object.keys(gitConfigList).forEach((key) => {
      const { name, email } = gitConfigList[key];
      console.log(`${key}\tuser.name:\t${name}\n\tuser.email:\t${email}`);
    });
  },

  set: (key?: string, value?: string) => {
    // svy git-config set:github name=qiyuor2
    // svy git-config set:github email=xxxx.com
    if (key && value) {
      const [k, v] = value.split('=');
      const prevConfig = getGitConfigByKey(key);
      const config = prevConfig ? { ...prevConfig, [k]: v } : { [k]: v };
      const gitConfigList = checkConfig().gitConfigList;
      gitConfigList[key] = config;
      updateConfig('gitConfigList', gitConfigList);
    }
  },

  use: (key?: string, keyWhenGlobal?: string) => {
    if (!key) {
      return;
    }
    if (keyWhenGlobal) {
      return actionMap.useGlobal(keyWhenGlobal);
    }
    const willUseConfig = getGitConfigByKey(key);
    setGitConfig(willUseConfig.name, willUseConfig.email);
  },

  useGlobal(key?: string) {
    const willUseConfig = getGitConfigByKey(key!);
    setGitConfig(willUseConfig.name, willUseConfig.email, true);
  },
};

export const gitConfig = createAction({
  name: 'git-config',
  args: { name: { type: String }, value: { type: String } },
  description: 'svy git-config 查看git仓库配置的用户名和邮箱',
  action({ name, value }) {
    name = isUndefined(name) ? 'get' : name;
    if (name.indexOf(':') !== -1) {
      // svy git-config set:github -> actionKey = set, configKey = github
      const [actionKey, configKey] = name.split(':');
      actionMap[actionKey as keyof typeof actionMap](configKey, value);
    } else {
      // svy git-config
      // svy git-config use github
      // svy git-config preset
      actionMap[name as keyof typeof actionMap](value);
    }
    console.log('\n');
  },
});
