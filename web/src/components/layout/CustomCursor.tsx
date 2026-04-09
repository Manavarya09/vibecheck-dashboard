"use client";

import { motion, useMotionValue, useSpring } from "framer-motion";
import { useEffect, useState } from "react";

export function CustomCursor() {
  const [visible, setVisible] = useState(false);
  const cursorX = useMotionValue(-100);
  const cursorY = useMotionValue(-100);

  const springConfig = { damping: 25, stiffness: 300, mass: 0.5 };
  const x = useSpring(cursorX, springConfig);
  const y = useSpring(cursorY, springConfig);

  useEffect(() => {
    // Only on desktop
    if (window.matchMedia("(pointer: coarse)").matches) return;

    const move = (e: MouseEvent) => {
      cursorX.set(e.clientX - 6);
      cursorY.set(e.clientY - 6);
      setVisible(true);
    };

    const leave = () => setVisible(false);

    window.addEventListener("mousemove", move);
    document.addEventListener("mouseleave", leave);
    return () => {
      window.removeEventListener("mousemove", move);
      document.removeEventListener("mouseleave", leave);
    };
  }, [cursorX, cursorY]);

  if (!visible) return null;

  return (
    <motion.div
      className="fixed top-0 left-0 w-3 h-3 rounded-full bg-terracotta/40 pointer-events-none z-[9999] mix-blend-multiply"
      style={{ x, y }}
    />
  );
}
