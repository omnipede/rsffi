using CsBindgen;
class Program {
    static void Main() {
        Console.WriteLine("hello, world!");
        int c = NativeMethods.add(1, 10);
        Console.WriteLine(c);
    }
}