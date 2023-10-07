# 🦀 Design Patterns in Rust

Repository to showcase various design patterns implemented in Rust.

---

## 🌍 Languages

- [🇬🇧 English](#english)
- [🇧🇷 Portuguese (BR)](#portuguese-br)
- [🇮🇹 Italian](#italian)

---

## 📚 Design Patterns

### Creational Patterns

- Singleton
- Builder
- Prototype
- Factory Method
- Abstract Factory

### Structural Patterns

- Adapter
- Bridge
- Composite
- Decorator
- Facade
- Flyweight
- Proxy

### Behavioral Patterns

- Chain of Responsibility
- Command
- Interpreter
- Iterator
- Mediator
- Memento
- Observer
- State
- Strategy
- Template Method
- Visitor

---

## 🇬🇧 English

### Creational Patterns

- **Singleton:** Ensures that a particular class has only one instance and provides a global point to this instance.
- **Builder:** Separates the construction of a complex object from its representation, allowing the same construction process to create different representations.
- **Prototype:** Create new objects by copying an existing object, known as the prototype.
- **Factory Method:** Define an interface for creating an object, but let subclasses decide which class to instantiate.
- **Abstract Factory:** Provide an interface for creating families of related or dependent objects without specifying their concrete classes.

### Structural Patterns

- **Adapter:** Allows objects with incompatible interfaces to work together.
- **Bridge:** Separates an object's abstraction from its implementation.
- **Composite:** Composes objects into tree structures to represent part-whole hierarchies.
- **Decorator:** Attach additional responsibilities to an object dynamically.
- **Facade:** Provides a simplified interface to a larger body of code.
- **Flyweight:** Reduces the cost of creating and manipulating a large number of similar objects.
- **Proxy:** Provides a substitute or placeholder for another object to control access to it.

### Behavioral Patterns

- **Chain of Responsibility:** Decouples sender from receiver by letting more than one object handle a request.
- **Command:** Turns a request into a standalone object containing information about the request.
- **Interpreter:** Provides a way to evaluate language grammar or expressions for particular languages.
- **Iterator:** Provides a way to access elements of a collection without exposing its underlying representation.
- **Mediator:** Reduces direct communication between classes by centralizing external communications.
- **Memento:** Captures and externalizes an object's internal state so it can be restored later.
- **Observer:** Allows an object to publish changes to its state so other objects can react accordingly.
- **State:** Allows an object to change its behavior when its internal state changes.
- **Strategy:** Allows selecting an implementation of an algorithm's interface at runtime.
- **Template Method:** Defines the structure of an algorithm, but delays some steps to subclasses.
- **Visitor:** Adds further operations to objects without having to modify them.

## 🇧🇷 Portuguese (BR)

### Padrões Criacionais

- **Singleton:** Garante que uma determinada classe tenha apenas uma instância e fornece um ponto global para esta instância.
- **Builder:** Separa a construção de um objeto complexo de sua representação, permitindo que o mesmo processo de construção crie diferentes representações.
- **Prototype:** Cria novos objetos copiando um objeto existente, conhecido como protótipo.
- **Factory Method:** Define uma interface para criar um objeto, mas deixa subclasses decidirem qual classe instanciar.
- **Abstract Factory:** Fornece uma interface para criar famílias de objetos relacionados ou dependentes sem especificar suas classes concretas.

### Padrões Estruturais

- **Adapter:** Permite que objetos com interfaces incompatíveis trabalhem juntos.
- **Bridge:** Separa a abstração de um objeto de sua implementação.
- **Composite:** Compõe objetos em estruturas de árvore para representar hierarquias parte-todo.
- **Decorator:** Anexa responsabilidades adicionais a um objeto dinamicamente.
- **Facade:** Fornece uma interface simplificada para um grande conjunto de código.
- **Flyweight:** Reduz o custo de criar e manipular um grande número de objetos semelhantes.
- **Proxy:** Fornece um substituto ou marcador de posição para outro objeto para controlar o acesso a ele.

### Padrões Comportamentais

- **Chain of Responsibility:** Desacopla o remetente do receptor, permitindo que mais de um objeto manipule uma solicitação.
- **Command:** Transforma uma solicitação em um objeto independente contendo informações sobre a solicitação.
- **Interpreter:** Fornece uma maneira de avaliar a gramática ou expressões de linguagem para linguagens específicas.
- **Iterator:** Fornece uma maneira de acessar os elementos de uma coleção sem expor sua representação subjacente.
- **Mediator:** Reduz a comunicação direta entre classes, centralizando comunicações externas.
- **Memento:** Captura e externaliza o estado interno de um objeto para que possa ser restaurado posteriormente.
- **Observer:** Permite que um objeto publique mudanças em seu estado para que outros objetos possam reagir de acordo.
- **State:** Permite que um objeto mude seu comportamento quando seu estado interno muda.
- **Strategy:** Permite selecionar uma implementação de uma interface de algoritmo em tempo de execução.
- **Template Method:** Define a estrutura de um algoritmo, mas adia algumas etapas para subclasses.
- **Visitor:** Adiciona operações adicionais aos objetos sem precisar modificá-los.

## 🇮🇹 Italian

### Modelli Creazionali

- **Singleton:** Garantisce che una determinata classe abbia solo un'istanza e fornisce un punto globale per questa istanza.
- **Builder:** Separa la costruzione di un oggetto complesso dalla sua rappresentazione, permettendo allo stesso processo di costruzione di creare diverse rappresentazioni.
- **Prototype:** Crea nuovi oggetti copiando un oggetto esistente, noto come prototipo.
- **Factory Method:** Definisce un'interfaccia per creare un oggetto, ma lascia che le sottoclassi decidano quale classe istanziare.
- **Abstract Factory:** Fornisce un'interfaccia per creare famiglie di oggetti correlati o dipendenti senza specificare le loro classi concrete.

### Modelli Strutturali

- **Adapter:** Permette a oggetti con interfacce incompatibili di lavorare insieme.
- **Bridge:** Separa l'astrazione di un oggetto dalla sua implementazione.
- **Composite:** Componi oggetti in strutture ad albero per rappresentare gerarchie parte-tutto.
- **Decorator:** Aggiunge responsabilità aggiuntive a un oggetto dinamicamente.
- **Facade:** Fornisce un'interfaccia semplificata ad un insieme più ampio di codice.
- **Flyweight:** Riduce il costo di creazione e manipolazione di un gran numero di oggetti simili.
- **Proxy:** Fornisce un sostituto o un segnaposto per un altro oggetto per controllarne l'accesso.

### Modelli Comportamentali

- **Chain of Responsibility:** Decoppia il mittente dal destinatario, permettendo a più di un oggetto di gestire una richiesta.
- **Command:** Trasforma una richiesta in un oggetto autonomo contenente informazioni sulla richiesta.
- **Interpreter:** Fornisce un modo per valutare la grammatica o le espressioni del linguaggio per linguaggi specifici.
- **Iterator:** Fornisce un modo per accedere agli elementi di una raccolta senza esporre la sua rappresentazione sottostante.
- **Mediator:** Riduce la comunicazione diretta tra classi centralizzando le comunicazioni esterne.
- **Memento:** Cattura ed esternalizza lo stato interno di un oggetto in modo che possa essere ripristinato in seguito.
- **Observer:** Permette a un oggetto di pubblicare cambiamenti nel suo stato in modo che altri oggetti possano reagire di conseguenza.
- **State:** Permette a un oggetto di cambiare il suo comportamento quando il suo stato interno cambia.
- **Strategy:** Permette di selezionare un'implementazione di un'interfaccia dell'algoritmo a runtime.
- **Template Method:** Definisce la struttura di un algoritmo, ma ritarda alcuni passaggi alle sottoclassi.
- **Visitor:** Aggiunge ulteriori operazioni agli oggetti senza doverli modificare.
