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

export type Conjugation = {
  __typename?: 'Conjugation';
  /** Pronoun used for the conjugation */
  pronoun: Pronoun;
  /** Conjugated verb in spanish */
  spanish: Scalars['String']['output'];
};

export enum Mood {
  ImperativoAfirmativo = 'IMPERATIVO_AFIRMATIVO',
  ImperativoNegativo = 'IMPERATIVO_NEGATIVO',
  Indicativo = 'INDICATIVO',
  Subjuntivo = 'SUBJUNTIVO'
}

export enum Pronoun {
  El = 'EL',
  Ellos = 'ELLOS',
  Nosotros = 'NOSOTROS',
  Tu = 'TU',
  Vosotros = 'VOSOTROS',
  Yo = 'YO'
}

export type QueryRoot = {
  __typename?: 'QueryRoot';
  /** get a verb */
  verb?: Maybe<Verb>;
  /** get a conjugated verb */
  verbTense?: Maybe<VerbTense>;
};


export type QueryRootVerbArgs = {
  infinitive: Scalars['String']['input'];
};


export type QueryRootVerbTenseArgs = {
  infinitive?: InputMaybe<Scalars['String']['input']>;
  tenses?: InputMaybe<Array<Tense>>;
};

export enum Tense {
  Condicional = 'CONDICIONAL',
  CondicionalPerfecto = 'CONDICIONAL_PERFECTO',
  Futuro = 'FUTURO',
  FuturoPerfecto = 'FUTURO_PERFECTO',
  Imperfecto = 'IMPERFECTO',
  Pluscuamperfecto = 'PLUSCUAMPERFECTO',
  Presente = 'PRESENTE',
  PresentePerfecto = 'PRESENTE_PERFECTO',
  Preterito = 'PRETERITO',
  PreteritoAnterior = 'PRETERITO_ANTERIOR'
}

export type Verb = {
  __typename?: 'Verb';
  /** Gerundio */
  gerundio: Scalars['String']['output'];
  /** English translation of the gerundio form */
  gerundioEnglish: Scalars['String']['output'];
  /** Infinitive form of the verb */
  infinitive: Scalars['String']['output'];
  /** English translation of the infinitive */
  infinitiveEnglish: Scalars['String']['output'];
  /** Tenses */
  tenses: Array<VerbTense>;
};

export type VerbTense = {
  __typename?: 'VerbTense';
  /** First person singular */
  conjugations: Array<Conjugation>;
  /** Infinitive form of the verb */
  infinitive: Scalars['String']['output'];
  /** Mood the verb has been conjugated */
  mood: Mood;
  /** Tense the verb has been conjugated */
  tense: Tense;
  /** Title of the combined tense and mood */
  title: Scalars['String']['output'];
  /** English form of the verb */
  verbEnglish?: Maybe<Scalars['String']['output']>;
};
