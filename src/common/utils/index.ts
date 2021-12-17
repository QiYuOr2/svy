export * from './bulkImport';
export * from './createAction';
export * from './checkConfig';

export const isUndefined = (val: unknown) => {
  return typeof val === 'undefined' ? true : typeof val === 'string' ? val === 'undefined' : false;
};
