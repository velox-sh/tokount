// simple Dart class example
class Animal {
    final String name;
    final int age;

    Animal(this.name, this.age);

    /* describe the animal */
    String describe() {
        return '$name is $age years old';
    }
}

void main() {
    final dog = Animal('Rex', 3);
    print(dog.describe());

    final cat = Animal('Whiskers', 5);
    print(cat.describe());
}
