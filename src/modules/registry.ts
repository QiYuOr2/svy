import { createAction } from '@/common';

export const registry = createAction({
  name: 'registry',
  args: {
    label: { type: String, required: true },
    source: { type: Number },
  },
  description: '切换NPM仓库',
  action({ label, source }) {
    console.log(label, source);
  },
});
