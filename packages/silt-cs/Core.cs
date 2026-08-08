using System;
using System.Runtime.InteropServices;

namespace Silt {
    public class CyclicDependencyException : Exception {
        public CyclicDependencyException(string msg) : base(msg) {}
    }

    public static class Graph {
        [DllImport("silt_core")]
        public static extern nuint silt_add(double v);
        
        [DllImport("silt_core")]
        public static extern void silt_set(nuint id, double v);
        
        [DllImport("silt_core")]
        public static extern double silt_get(nuint id);
        
        [DllImport("silt_core")]
        public static extern void silt_link(nuint src, nuint dst);
    }
}