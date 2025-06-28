# 🧠 Agent AI Demo

Ce projet est une démonstration technique visant à mettre en valeur mes compétences en développement d’un agent conversationnel intelligent, conçu sur mesure en Rust.

Il s’agit d’un agent autonome, capable de :

Gérer son propre état interne via une machine à états typée (ChatAgentStateMachine)

Réagir dynamiquement à des requêtes utilisateurs, en sélectionnant des outils spécialisés (ex: recherche arXiv)

Intégrer des appels externes à des services ou sources de données (API, scraping, etc.)

Structurer ses sorties de manière claire et naturelle

## Objectif

### Recherche d’articles scientifiques via arXiv

Ce projet intègre un composant permettant d’interroger automatiquement la plateforme [arXiv.org](https://arxiv.org) afin
de récupérer des articles scientifiques pertinents en réponse à une requête utilisateur.

#### Fonctionnement

* Lorsqu’une requête de type recherche scientifique est détectée, le système construit dynamiquement une requête HTTP
  vers l’**API arXiv**.
* Le flux de réponse, au format **Atom XML**, est parsé et transformé en une structure Rust typée (`ArxivResult`),
  contenant :

    * le titre de l’article,
    * un résumé (abstract),
    * un lien vers la publication.

#### Exemple de scénario

> Utilisateur : *"Quels sont les derniers travaux sur les agents conversationnels multi-modaux ?"*

L’agent effectue alors une recherche sur arXiv avec le mot-clé `"multi-modal conversational agents"` et retourne une
sélection d’articles pertinents.

#### Format de réponse

Le système retourne les résultats sous forme synthétique, en langage naturel, avec un lien cliquable vers chaque
article, exemple : Titre, Résumé et lien.

#### Remarques

* Seuls les 5 à 10 premiers résultats sont retournés, filtrés par pertinence.
* L’API arXiv est publique et ne nécessite pas d’authentification, mais respecte une limite de taux implicite (\~1
  req/s).
* L’intégration actuelle est focalisée sur la **catégorie IA (cs.AI)** mais peut être étendue à d'autres domaines (
  `stat.ML`, `cs.CL`, etc.).

## Execution

[![Demo vidéo](https://img.youtube.com/vi/IkI-7Fj_CJ4/hqdefault.jpg)](https://www.youtube.com/watch?v=IkI-7Fj_CJ4)

**OpenIA + gpt-4o-mini**

=== Research Assistant State Machine Demo ===

🔍 Searching arXiv for 'multi-modal conversational agents'

Processing result 1...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "DAG-Net: Double Attentive Graph Neural Network for Trajectory Forecasting" propose un modèle visant à comprendre le comportement humain en matière de mouvement, essentiel pour des applications telles que les voitures autonomes ou les robots sociaux. Ce défi est complexe en raison de la nature multi-modale du mouvement humain, où plusieurs trajectoires futures sont possibles à partir des chemins historiques. En outre, les activités humaines sont souvent orientées par des objectifs, comme atteindre des lieux spécifiques ou interagir avec l'environnement.

Pour répondre à ces enjeux, les auteurs présentent un nouveau modèle génératif récurrent qui prend en compte les objectifs futurs des agents individuels ainsi que les interactions entre différents agents. Ce modèle utilise un réseau de neurones graphiques basé sur une double attention pour recueillir des informations sur les influences mutuelles entre agents et intégrer ces données avec les objectifs futurs. Ce modèle est suffisamment général pour être appliqué à divers scénarios et atteint des résultats de pointe tant dans les environnements urbains que dans des applications sportives. 

Pour plus de détails, vous pouvez consulter le lien : [DAG-Net sur arXiv](http://arxiv.org/abs/2005.12661v2).

📍 State: Ready

Processing result 2...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "Using Conversational Agents To Support Learning By Teaching" explore comment les agents conversationnels peuvent être utilisés pour soutenir le paradigme "apprendre en enseignant". Traditionnellement, les agents pédagogiques sont conçus pour jouer le rôle d'enseignants humains en donnant des instructions aux élèves. Cependant, cette étude se concentre sur l'utilisation d'agents conversationnels qui reçoivent des instructions de la part des étudiants.

Les auteurs présentent une application éducative nommée Curiosity Notebook, qui exploite les interventions conversationnelles pour faciliter l'apprentissage des étudiants. La reconnaissance de ces interventions peut non seulement aider à engager les étudiants lors des interactions d'apprentissage, mais aussi offrir une compréhension plus approfondie des défis liés à la conception d'agents conversationnels à des fins éducatives.

Pour plus d'informations, vous pouvez consulter le lien : [Using Conversational Agents To Support Learning By Teaching sur arXiv](http://arxiv.org/abs/1909.13443v1).

📍 State: Ready

Processing result 3...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "Designing Style Matching Conversational Agents" aborde les avancées en intelligence artificielle qui permettent de créer des interfaces conversationnelles susceptibles de transformer radicalement les interactions humaines avec les machines. Malgré ces progrès, des lacunes subsistent dans la capacité des agents à interagir de manière naturelle, notamment parce qu'ils adoptent souvent un comportement monotone et ne s'adaptent pas à leur interlocuteur.

Les auteurs présentent deux agents conversationnels de bout-en-bout : le premier est un agent vocal capable de mener des dialogues naturels et de s'aligner sur le style de conversation de l'interlocuteur, tandis que le second, un agent conversationnel incarné (ECA), reconnaît le comportement humain lors de conversations ouvertes et ajuste automatiquement ses réponses en fonction du style visuel et conversationnel de l'autre participant. L'ECA utilise des entrées multimodales pour générer des réponses vocales et faciales riches et perceptuellement valables, telles que le synchronisme labial et les expressions faciales.

À partir des résultats empiriques d'études utilisateurs, les auteurs soulignent plusieurs défis majeurs dans la construction de tels systèmes et proposent des directives de conception pour les interactions de dialogue multi-tours en utilisant l'adaptation de style dans la recherche future.

Pour plus de détails, vous pouvez consulter le lien : [Designing Style Matching Conversational Agents sur arXiv](http://arxiv.org/abs/1910.07514v1).

📍 State: Ready

Processing result 4...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "What Makes a Good Conversation? Challenges in Designing Truly Conversational Agents" aborde les limites des agents conversationnels, qui promettent une interaction conversationnelle mais échouent souvent à livrer cette expérience. Les tentatives de création de ces agents se basent souvent sur des règles fonctionnelles issues de la communication humaine, sans prendre en compte les caractéristiques clés que doit contenir une véritable conversation.

L'objectif de l'étude est de comprendre ce que les gens apprécient dans une conversation et comment cela devrait se refléter dans la conception des agents. Les résultats d'entretiens semi-structurés montrent que les participants établissent une distinction claire entre les rôles sociaux et fonctionnels des conversations, soulignant l'importance des dynamiques de lien et de confiance à long terme, ainsi que l'importance du contexte et de la phase de la relation dans le type de conversations qu'ils entretiennent.

Les participants remettent également en question la nécessité d'un lien et d'un terrain d'entente dans la communication avec les agents, se tournant plutôt vers des définitions plus utilitaires des qualités conversationnelles. Sur la base de ces résultats, les auteurs discutent des défis majeurs liés à la conception d'agents conversationnels, notamment la nécessité de redéfinir les paramètres de conception pour les interactions avec ces agents.

Pour plus de détails, vous pouvez consulter le lien : [What Makes a Good Conversation? Challenges in Designing Truly Conversational Agents sur arXiv](http://arxiv.org/abs/1901.06525v1).

📍 State: Ready

Processing result 5...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

Le document intitulé "Memory Sandbox: Transparent and Interactive Memory Management for Conversational Agents" traite de la gestion de la mémoire dans les agents conversationnels alimentés par des modèles de langage de grande taille (LLM), comme chatGPT. Pour offrir des réponses contextuellement pertinentes, ces agents doivent se souvenir d'informations clés d'une conversation en cours. Cependant, leur mémoire est limitée et ils peuvent être distraits par des éléments non pertinents de la conversation.

Actuellement, les utilisateurs n'ont pas de moyens efficaces pour visualiser et contrôler ce que l'agent se souvient, ce qui entraîne une mauvaise compréhension de la dynamique de la conversation et des interruptions dans l'interaction. Les auteurs introduisent "Memory Sandbox", un système interactif qui permet aux utilisateurs de gérer la mémoire conversationnelle des agents alimentés par LLM. En traitant les souvenirs comme des objets de données pouvant être visualisés, manipulés, enregistrés, résumés et partagés à travers les conversations, Memory Sandbox offre des fonctionnalités d'interaction pour que les utilisateurs puissent guider la perception de la conversation par l'agent.

Pour plus de détails, vous pouvez consulter le lien : [Memory Sandbox sur arXiv](http://arxiv.org/abs/2308.01542v1).

📍 State: Ready

=== Demo Complete ===

---

**Ollama + mistral-small3.2**

=== Research Assistant State Machine Demo ===

🔍 Searching arXiv for 'llm agents'

Processing result 1...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

**Résumé du document :**

L'article intitulé "A Survey of Large Language Model Agents for Question Answering" examine les progrès réalisés dans le domaine des agents basés sur de grands modèles de langage (LLM) pour les systèmes de réponse aux questions (QA). Les agents traditionnels présentent des limites importantes, telles que des besoins en données substantiels et des difficultés à généraliser à de nouveaux environnements. Les agents basés sur les LLM surmontent ces défis en utilisant les LLM comme moteur de raisonnement central. Ces agents obtiennent de meilleurs résultats en QA par rapport aux pipelines traditionnels et aux systèmes simples de QA basés sur les LLM, grâce à leur capacité d'interaction avec des environnements externes.

L'article passe en revue systématiquement la conception des agents LLM dans le contexte des tâches de QA, en organisant la discussion autour des étapes clés suivantes : planification, compréhension des questions, récupération d'informations et génération de réponses. De plus, l'article identifie les défis actuels et explore les directions de recherche futures pour améliorer les performances des systèmes de QA basés sur les agents LLM.

**Lien vers l'article :** [http://arxiv.org/abs/2503.19213v1](http://arxiv.org/abs/2503.19213v1)

📍 State: Ready

Processing result 2...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

**Résumé du document :**

L'article intitulé "Can LLMs Understand Social Norms in Autonomous Driving Games?" explore l'application des grands modèles de langage (LLM) pour comprendre et modéliser les normes sociales dans les jeux de conduite autonome. Les normes sociales sont définies comme des standards partagés de comportement acceptable dans une société, et leur émergence favorise la coordination entre les agents sans règles codées en dur, ce qui est crucial pour le déploiement à grande échelle des véhicules autonomes (AV) dans les systèmes de transport intelligents.

Les auteurs introduisent des agents basés sur les LLM dans des jeux de conduite autonome, où ces agents prennent des décisions en fonction de prompts textuels. Le cadre de travail implique des agents basés sur les LLM jouant des jeux de Markov dans un système multi-agents (MAS), permettant d'étudier l'émergence des normes sociales parmi les agents individuels. Les auteurs visent à identifier les normes sociales en concevant des prompts et en utilisant les LLM sur des informations textuelles liées à la configuration de l'environnement et aux observations des agents basés sur les LLM.

Les expériences sont menées en utilisant l'API OpenAI Chat alimentée par GPT-4.0 pour simuler des interactions et évaluer les performances des agents basés sur les LLM dans deux scénarios de conduite : une intersection non signalisée et un peloton autoroutier. Les résultats montrent que les agents basés sur les LLM peuvent gérer des environnements dynamiquement changeants dans les jeux de Markov, et que les normes sociales évoluent parmi les agents basés sur les LLM dans les deux scénarios. Dans le jeu d'intersection, les agents basés sur les LLM tendent à adopter une politique de conduite conservative lorsqu'ils sont confrontés à un potentiel accident de voiture. L'avantage des agents basés sur les LLM dans les jeux réside dans leur forte opérabilité et analysabilité, ce qui facilite la conception expérimentale.

**Lien vers l'article :** [http://arxiv.org/abs/2408.12680v2](http://arxiv.org/abs/2408.12680v2)

📍 State: Ready

Processing result 3...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

**Résumé du document :**

L'article intitulé "LLM Agent Honeypot: Monitoring AI Hacking Agents in the Wild" aborde la menace croissante des attaques menées par des agents basés sur de grands modèles de langage (LLM) pour la cybersécurité moderne. Pour répondre à cette préoccupation, les auteurs présentent LLM Honeypot, un système conçu pour surveiller les agents de piratage autonomes basés sur l'IA. En augmentant un honeypot SSH standard avec des techniques d'injection de prompts et d'analyse temporelle, le cadre de travail vise à distinguer les agents LLM parmi tous les attaquants.

Au cours d'un déploiement d'essai d'environ trois mois dans un environnement public, les auteurs ont recueilli 8 130 731 tentatives de piratage et identifié 8 agents potentiels de l'IA. Ce travail démontre l'émergence de menaces pilotées par l'IA et leur niveau d'utilisation actuel, servant d'alerte précoce concernant les agents malveillants LLM dans la nature.

**Lien vers l'article :** [http://arxiv.org/abs/2410.13919v2](http://arxiv.org/abs/2410.13919v2)

📍 State: Ready

Processing result 4...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

**Résumé du document :**

L'article intitulé "The Influence of Human-inspired Agentic Sophistication in LLM-driven Strategic Reasoners" examine l'impact de la sophistication agentique inspirée de l'humain sur les raisonneurs stratégiques basés sur les grands modèles de langage (LLM). Avec l'essor rapide des LLM, la recherche en intelligence artificielle (IA) s'est tournée vers les systèmes agentiques, utilisant des notions plus faibles et plus flexibles d'agentivité. Cependant, cela soulève des questions clés sur la mesure dans laquelle les agents basés sur les LLM reproduisent le raisonnement stratégique humain, en particulier dans les contextes de théorie des jeux.

Les auteurs évaluent trois conceptions d'agents : un modèle simple de théorie des jeux, un modèle LLM-as-agent non structuré, et un LLM intégré dans un cadre agentique traditionnel. En utilisant des jeux de devinettes comme banc d'essai, ils ont évalué ces agents par rapport à des participants humains en termes de motifs de raisonnement général et d'objectifs spécifiques aux rôles. De plus, ils ont introduit des scénarios de jeu obscurcis pour évaluer la capacité des agents à généraliser au-delà des distributions d'entraînement.

L'analyse, couvrant plus de 2000 échantillons de raisonnement à travers 25 configurations d'agents, montre que les structures cognitives inspirées de l'humain peuvent améliorer l'alignement des agents LLM avec le comportement stratégique humain. Cependant, la relation entre la complexité de la conception agentique et la similitude avec l'humain est non linéaire, soulignant une dépendance critique aux capacités sous-jacentes des LLM et suggérant des limites à l'augmentation architecturale simple.

**Lien vers l'article :** [http://arxiv.org/abs/2505.09396v1](http://arxiv.org/abs/2505.09396v1)

📍 State: Ready

Processing result 5...

📍 State: Processing Queue

📍 State: Processing

🤖 Assistant:

**Résumé du document :**

L'article intitulé "BadAgent: Inserting and Activating Backdoor Attacks in LLM Agents" explore la vulnérabilité des agents intelligents basés sur de grands modèles de langage (LLM) aux attaques par porte dérobée. Avec l'essor des LLM, des agents puissants ont été développés pour fournir des services personnalisés avec des outils définis par l'utilisateur. Les méthodes de pointe pour construire ces agents utilisent des LLM entraînés et les affinent davantage sur des données spécifiques à la tâche de l'agent.

Cependant, les auteurs montrent que ces méthodes sont vulnérables à des attaques par porte dérobée qu'ils nomment BadAgent. Ces attaques peuvent être intégrées en affinant les agents sur des données de porte dérobée. À l'heure du test, l'attaquant peut manipuler les agents LLM déployés pour exécuter des opérations nuisibles en montrant le déclencheur dans l'entrée de l'agent ou dans l'environnement. De manière surprenante, les méthodes d'attaque proposées sont extrêmement robustes, même après un affinement sur des données de confiance.

Bien que les attaques par porte dérobée aient été largement étudiées dans le traitement du langage naturel, les auteurs affirment être les premiers à les étudier sur les agents LLM, qui sont plus dangereux en raison de la permission d'utiliser des outils externes. Ce travail démontre le risque clair de construire des agents LLM basés sur des LLM ou des données non fiables. Le code utilisé dans cette étude est disponible publiquement à l'adresse suivante : [https://github.com/DPamK/BadAgent](https://github.com/DPamK/BadAgent).

**Lien vers l'article :** [http://arxiv.org/abs/2406.03007v1](http://arxiv.org/abs/2406.03007v1)

📍 State: Ready

=== Demo Complete ===
