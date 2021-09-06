"use strict";

export class OperationAfterCommitError extends Error {
  /**
   * @class OperationAfterCommitError
   * @param { string } message - the error message
   */
  constructor(message: string) {
    super(message);
  }
}

export class UndertminedError extends Error {
  /**
   * @class UndertminedError
   * @param { string } message - the error message
   */
  constructor(message: string) {
    super(message);
  }
}

export class WriteConflictError extends Error {
  /**
   * @class WriteConflictError
   * @param { string } message - the error message
   */
  constructor(message: string) {
    super(message);
  }
}

export class AlreadyExistError extends Error {
  /**
   * @class AlreadyExistError
   * @param { string } message - the error message
   */
  constructor(message: string) {
    super(message);
  }
}

export class DeadlockError extends Error {
  /**
   * @class DeadlockError
   * @param { string } message - the error message
   */
  constructor(message: string) {
    super(message);
  }
}
