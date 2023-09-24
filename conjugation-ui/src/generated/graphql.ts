export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export type ConjugatedVerb = {
  __typename?: 'ConjugatedVerb';
  /** Third person singular */
  el: Scalars['String']['output'];
  /** Third person plural */
  ellos: Scalars['String']['output'];
  /** Infinitive form of the verb */
  infinitive: Scalars['String']['output'];
  /** First person plural */
  nosotros: Scalars['String']['output'];
  /** Tense the verb has been conjugated */
  tense: Tense;
  /** Second person singular */
  tu: Scalars['String']['output'];
  /** English form of the verb */
  verbEnglish: Scalars['String']['output'];
  /** Second person plural */
  vosotros: Scalars['String']['output'];
  /** First person singular */
  yo: Scalars['String']['output'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  /** get a random verb */
  randomVerb?: Maybe<ConjugatedVerb>;
  /** search for a verb */
  searchVerb?: Maybe<ConjugatedVerb>;
};


export type QueryRootSearchVerbArgs = {
  infinitive: Scalars['String']['input'];
  tense: Tense;
};

export enum Tense {
  Futuro = 'FUTURO',
  Imperfecto = 'IMPERFECTO',
  Presente = 'PRESENTE',
  Preterito = 'PRETERITO'
}
